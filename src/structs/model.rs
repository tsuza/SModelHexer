use std::{fs::File, io::Read, path::PathBuf};

const MODEL_PATH_OFFSET: u64 = 0xC;
const NUMBER_OF_MATERIAL_PATHS_OFFSET: u64 = 0xD4;
const ADDRESS_OF_MODEL_PATH_OFFSET: u64 = 0x15C;
const FILE_END_ADDRESS_OFFSET: u64 = 0x4C;
const ADDRESS_OF_MATERIAL_PATHS_OFFSET: u64 = NUMBER_OF_MATERIAL_PATHS_OFFSET + 0x4;
const SURFACE_PROP_OFFSET_OFFSET: u64 = 0x134;

pub struct Model {
    // This shouldn't be accessed directly, but we're leaving it public in case it's really necessary.
    pub model_raw_data: Vec<u8>,
}

impl Model {
    /* TODO: Replace file_path from &str to Path ( or PathBuf ) */
    pub fn new(file_path: &PathBuf) -> Model {
        let mut file = File::open(file_path).unwrap();

        let mut model: Vec<u8> = Vec::new();

        file.read_to_end(&mut model).unwrap();

        Model {
            model_raw_data: model,
        }
    }

    pub fn save(&self, file_path: &PathBuf) {
        std::fs::write(&file_path, &self.model_raw_data).unwrap();
    }
}

impl Model {
    pub fn get_model_path(&self) -> String {
        let mut model_path: Vec<u8> = Vec::new();

        let mut current_position = MODEL_PATH_OFFSET;

        loop {
            let current_byte = self
                .model_raw_data
                .get(current_position as usize)
                .unwrap()
                .clone();

            // It means that we've read the entire file path.
            if current_byte == 0x0 {
                break;
            }

            model_path.push(current_byte);

            current_position += 0x1;
        }

        String::from_utf8(model_path).unwrap()
    }

    pub fn set_model_path(&mut self, new_model_path: &str) {
        let model_path_length = self.get_model_path().len();

        let mut new_model_path_array: [u8; 64] = [0; 64];

        new_model_path_array[..new_model_path.len()].copy_from_slice(new_model_path.as_bytes());

        self.model_raw_data.splice(
            MODEL_PATH_OFFSET as usize..MODEL_PATH_OFFSET as usize + 64,
            new_model_path_array,
        );

        /*
        if model_path_length as i32 - new_model_path.len() as i32 > 0 {
            self.model_raw_data.splice(
                MODEL_PATH_OFFSET as usize + new_model_path.len()
                    ..MODEL_PATH_OFFSET as usize + new_model_path.len(),
                vec![0x0u8; model_path_length as usize - new_model_path.len() as usize],
            );
        }
        */

        /*

        let mut model_path_offset: [u8; 4] = [0; 4];

        model_path_offset.copy_from_slice(
            &self.model_raw_data
                [ADDRESS_OF_MODEL_PATH_OFFSET as usize..ADDRESS_OF_MODEL_PATH_OFFSET as usize + 4],
        );

        let model_path_offset = bytes_to_u32(&model_path_offset) + 0x1;

        self.model_raw_data.splice(
            model_path_offset as usize..model_path_offset as usize + model_path_length as usize,
            new_model_path.as_bytes().to_owned(),
        );

        // We need to fix all of the offsets now...

        let diff = new_model_path.len() as i32 - model_path_length as i32;

        let mut surface_prop_offset: [u8; 4] = [0; 4];

        surface_prop_offset.copy_from_slice(
            &self.model_raw_data
                [SURFACE_PROP_OFFSET_OFFSET as usize..SURFACE_PROP_OFFSET_OFFSET as usize + 4],
        );

        // This is the address to the address to the material path ( 2 layers of addresses )
        let surface_prop_offset = bytes_to_u32(&surface_prop_offset);

        println!("{surface_prop_offset}");

        let new_offset = surface_prop_offset as i32 + diff;

        let new_offset: [u8; 4] = [
            (new_offset & 0xFF) as u8,
            ((new_offset >> 8) & 0xFF) as u8,
            ((new_offset >> 16) & 0xFF) as u8,
            ((new_offset >> 24) & 0xFF) as u8,
        ];

        self.model_raw_data.splice(
            SURFACE_PROP_OFFSET_OFFSET as usize..SURFACE_PROP_OFFSET_OFFSET as usize + 4,
            new_offset,
        );

        let number_of_material_paths = self.get_materials_number();

        let mut first_material_path_address_offset: [u8; 4] = [0; 4];

        first_material_path_address_offset.copy_from_slice(
            &self.model_raw_data[ADDRESS_OF_MATERIAL_PATHS_OFFSET as usize
                ..ADDRESS_OF_MATERIAL_PATHS_OFFSET as usize + 4],
        );

        let first_material_path_address_offset = bytes_to_u32(&first_material_path_address_offset);

        // Now that we have the offset of the subsection, we can iterate over it to get to every single path and change them accordingly.
        for position in 0..number_of_material_paths {
            // We need to get the length of the path first.
            let old_material_offset_offset =
                first_material_path_address_offset + position as u32 * 0x4;

            let mut old_material_path_offset: [u8; 4] = [0; 4];
            old_material_path_offset.copy_from_slice(
                &self.model_raw_data[old_material_offset_offset as usize
                    ..old_material_offset_offset as usize + 0x4],
            );

            let old_material_path_offset = bytes_to_u32(&old_material_path_offset);

            // Now we need to fix the offset of the next material path.
            let new_offset = (old_material_path_offset as i32 + diff) as u32;

            let new_offset: [u8; 4] = [
                (new_offset & 0xFF) as u8,
                ((new_offset >> 8) & 0xFF) as u8,
                ((new_offset >> 16) & 0xFF) as u8,
                ((new_offset >> 24) & 0xFF) as u8,
            ];

            self.model_raw_data.splice(
                old_material_offset_offset as usize..old_material_offset_offset as usize + 0x4,
                new_offset,
            );
        }

        self.update_ending_file_offset();

        */
    }
}

impl Model {
    pub fn get_materials_number(&self) -> u8 {
        self.model_raw_data
            .get(NUMBER_OF_MATERIAL_PATHS_OFFSET as usize)
            .unwrap()
            .clone()
    }

    pub fn set_materials_number(&mut self, number_of_materials: u8) {
        // Afaik the limit of material paths is 32 ( although Ficool has found a way to go past that ),
        // so one byte will be enough.
        self.model_raw_data.insert(
            NUMBER_OF_MATERIAL_PATHS_OFFSET as usize,
            number_of_materials,
        );
    }

    pub fn get_material_paths(&self) -> Vec<String> {
        let mut material_paths: Vec<String> = Vec::new();

        let number_of_material_paths = self.get_materials_number();

        let mut first_material_path_address_offset: [u8; 4] = [0; 4];

        first_material_path_address_offset.copy_from_slice(
            &self.model_raw_data[ADDRESS_OF_MATERIAL_PATHS_OFFSET as usize
                ..ADDRESS_OF_MATERIAL_PATHS_OFFSET as usize + 4],
        );

        let first_material_path_address_offset = bytes_to_u32(&first_material_path_address_offset);

        // Now that we have the offset of the subsection, we can iterate over it to get to every single path and change them accordingly.
        for position in 0..number_of_material_paths {
            // We need to get the length of the path first.
            let old_material_offset_offset =
                first_material_path_address_offset + position as u32 * 0x4;

            let mut old_material_path_offset: [u8; 4] = [0; 4];
            old_material_path_offset.copy_from_slice(
                &self.model_raw_data[old_material_offset_offset as usize
                    ..old_material_offset_offset as usize + 0x4],
            );

            let old_material_path_offset = bytes_to_u32(&old_material_path_offset);

            println!("1");

            let material_path = String::from_utf8(read_until_null_terminator(
                &self.model_raw_data,
                old_material_path_offset,
            ))
            .unwrap();

            println!("1.5");

            material_paths.push(material_path);
        }

        material_paths
    }

    pub fn set_material_paths(&mut self, new_material_paths: &Vec<String>) {
        let number_of_material_paths = self.get_materials_number();

        let mut first_material_path_address_offset: [u8; 4] = [0; 4];

        first_material_path_address_offset.copy_from_slice(
            &self.model_raw_data[ADDRESS_OF_MATERIAL_PATHS_OFFSET as usize
                ..ADDRESS_OF_MATERIAL_PATHS_OFFSET as usize + 0x4],
        );

        let first_material_path_address_offset = bytes_to_u32(&first_material_path_address_offset);

        for (position, new_material_path) in new_material_paths.iter().enumerate() {
            let mut old_material_path_offset: [u8; 4] = [0; 4];

            old_material_path_offset.copy_from_slice(
                &self.model_raw_data[first_material_path_address_offset as usize + position * 0x4
                    ..first_material_path_address_offset as usize + position * 0x4 + 0x4],
            );

            let old_material_path_offset = bytes_to_u32(&old_material_path_offset);

            let old_material_path_length =
                read_until_null_terminator(&self.model_raw_data, old_material_path_offset).len();

                println!("L: {}", String::from_utf8(read_until_null_terminator(&self.model_raw_data, old_material_path_offset)).unwrap());

            self.model_raw_data.splice(
                old_material_path_offset as usize
                    ..old_material_path_offset as usize + old_material_path_length,
                new_material_path.as_bytes().to_owned(),
            );
            
            if position < number_of_material_paths as usize - 1 {
                let next_offset = first_material_path_address_offset + (position as u32 * 0x4) + 0x4;

                let new_offset = old_material_path_offset as i32 + new_material_path.len() as i32 + 1;

                println!("Next offset: {next_offset}");                
                println!("New offset: {new_offset}");

                let new_offset: [u8; 4] = [
                    (new_offset & 0xFF) as u8,
                    ((new_offset >> 8) & 0xFF) as u8,
                    ((new_offset >> 16) & 0xFF) as u8,
                    ((new_offset >> 24) & 0xFF) as u8,
                ];


                self.model_raw_data.splice(
                    next_offset as usize..next_offset as usize + 0x4,
                    new_offset,
                );
            }
        }

        self.update_ending_file_offset();
    }

    /*
    pub fn set_material_paths(&mut self, new_material_paths: &Vec<String>) {
        // First we need to get the offset to the subsection where the offsets of the material paths reside.

        let number_of_material_paths = self.get_materials_number();

        let mut first_material_path_address_offset: [u8; 4] = [0; 4];

        first_material_path_address_offset.copy_from_slice(
            &self.model_raw_data[ADDRESS_OF_MATERIAL_PATHS_OFFSET as usize
                ..ADDRESS_OF_MATERIAL_PATHS_OFFSET as usize + 4],
        );

        let first_material_path_address_offset = bytes_to_u32(&first_material_path_address_offset);

        // Now that we have the offset of the subsection, we can iterate over it to get to every single path and change them accordingly.
        for (position, new_material_path) in new_material_paths.iter().enumerate() {
            // We need to get the length of the path first.
            let old_material_offset_offset =
                first_material_path_address_offset + position as u32 * 0x4;

            let mut old_material_path_offset: [u8; 4] = [0; 4];
            old_material_path_offset.copy_from_slice(
                &self.model_raw_data[old_material_offset_offset as usize
                    ..old_material_offset_offset as usize + 0x4],
            );

            let old_material_path_offset = bytes_to_u32(&old_material_path_offset);

            let old_material_path_length =
                read_until_null_terminator(&self.model_raw_data, old_material_path_offset).len();

            println!("{old_material_path_length} l");

            self.model_raw_data.splice(
                old_material_path_offset as usize
                    ..old_material_path_offset as usize + old_material_path_length as usize,
                new_material_path.as_bytes().to_owned(),
            );

            println!("4");

            // Now we need to fix the offset of the next material path.
            if number_of_material_paths > position as u8 + 1 {
                let new_offset = old_material_path_offset + new_material_path.len() as u32;

                let new_offset: [u8; 4] = [
                    (new_offset & 0xFF) as u8,
                    ((new_offset >> 8) & 0xFF) as u8,
                    ((new_offset >> 16) & 0xFF) as u8,
                    ((new_offset >> 24) & 0xFF) as u8,
                ];

                self.model_raw_data.splice(
                    (old_material_offset_offset + ((position as u32 + 1 ) * 0x4)) as usize..(old_material_offset_offset as usize + ((position + 1 ) * 0x4) + 0x4) as usize + 1,
                    new_offset,
                );
            }
        }

        //self.set_materials_number(new_material_paths.len() as u8);

        self.update_ending_file_offset();
    }
    */
}

impl Model {
    pub fn update_all_offsets(&mut self, diff: i32) {}
}

impl Model {
    pub fn update_ending_file_offset(&mut self) {
        let ending_offset = self.model_raw_data.len() as u64;

        let ending_offset: [u8; 4] = [
            (ending_offset & 0xFF) as u8,
            ((ending_offset >> 8) & 0xFF) as u8,
            ((ending_offset >> 16) & 0xFF) as u8,
            ((ending_offset >> 24) & 0xFF) as u8,
        ];

        self.model_raw_data.splice(
            FILE_END_ADDRESS_OFFSET as usize..FILE_END_ADDRESS_OFFSET as usize + 4,
            ending_offset,
        );
    }
}

fn bytes_to_u32(bytes: &[u8; 4]) -> u32 {
    bytes[0] as u32 | (bytes[1] as u32) << 8 | (bytes[2] as u32) << 16 | (bytes[3] as u32) << 24
}

fn read_until_null_terminator(model: &Vec<u8>, offset: u32) -> Vec<u8> {
    let mut data: Vec<u8> = Vec::new();

    let mut current_position = offset;

    loop {
        let current_byte = model.get(current_position as usize).unwrap().clone();

        if current_byte == 0x0 {
            break;
        }

        data.push(current_byte);

        current_position += 0x1;
    }

    data
}
