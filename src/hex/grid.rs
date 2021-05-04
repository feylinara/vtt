use crate::fgl::{self, Bindable};
use image::GenericImageView;

pub struct HexGridBuilder<'a> {
    point_up: bool,
    tiles: &'a [image::DynamicImage],
    dimensions: (u32, u32),
    grid_contents: Option<Vec<isize>>,
}

impl<'a> Default for HexGridBuilder<'a> {
    fn default() -> Self {
        Self {
            point_up: false,
            tiles: &[],
            dimensions: (0, 0),
            grid_contents: None,
        }
    }
}

impl<'a> HexGridBuilder<'a> {
    pub fn with_tiles(mut self, tiles: &'a [image::DynamicImage]) -> Self {
        self.tiles = tiles;
        self
    }
    pub fn with_grid_contents(mut self, gc: &[Option<usize>]) -> Self {
        self.grid_contents = Some(
            gc.iter()
                .map(|x| x.map(|x| x as isize).unwrap_or(-1))
                .collect(),
        );
        self
    }
    pub fn build(mut self) -> HexGrid {
        assert_eq!(
            (self.dimensions.0 * self.dimensions.1) as usize,
            self.grid_contents
                .as_ref()
                .map(|x| x.len())
                .unwrap_or((self.dimensions.0 * self.dimensions.1) as usize)
        );
        let tile_size = self
            .tiles
            .iter()
            .map(|image| u32::max(image.dimensions().0, image.dimensions().1))
            .max()
            .unwrap_or(0);
        let mut texture = fgl::texture::Texture2D::with_dimensions(
            tile_size as i32 * self.tiles.len() as i32,
            tile_size as i32,
            fgl::texture::Format::Rgba,
        );
        for (n, image) in self.tiles.iter().enumerate() {
            texture.replace_rect((n as u32 * tile_size) as i32, 0, image.clone());
        }

        let vao = fgl::VertexAttribObject::new();

        let mut vbos: [fgl::VertexBuffer; 3] = fgl::VertexBuffer::new_array();
        vbos[0].alloc_with(
            &fgl::consts::QUAD,
            fgl::AccessFrequency::Static,
            fgl::AccessType::Draw,
        );
        vao.vertex_attribute_array(
            &vbos[0],
            fgl::VertexAttribArray::<f32>::with_id(0).with_components_per_value(2),
        );

        let offsets = super::grid_coords(self.dimensions.0, self.dimensions.1, tile_size as f32);
        vbos[1].alloc_with(
            &offsets,
            fgl::AccessFrequency::Static,
            fgl::AccessType::Draw,
        );
        vao.vertex_attribute_array(
            &vbos[1],
            fgl::VertexAttribArray::<f32>::with_id(1)
                .with_components_per_value(2)
                .with_divisor(6),
        );

        let dims = self.dimensions;
        self.grid_contents = Some(
            self.grid_contents
                .unwrap_or_else(|| vec![0isize; (dims.0 * dims.1) as usize]),
        );
        vao.vertex_attribute_array(
            &vbos[2],
            fgl::VertexAttribArray::<f32>::with_id(2).with_divisor(6),
        );
        vbos[2].alloc_with(
            &(self
                .grid_contents
                .as_ref()
                .unwrap()
                .iter()
                .map(|x| *x as f32)
                .collect::<Vec<_>>()),
            fgl::AccessFrequency::Dynamic,
            fgl::AccessType::Draw,
        );

        HexGrid {
            dimensions: self.dimensions,
            tile_size,
            vbos,
            vao,
            texture,
            tilecount: self.tiles.len() as u32,
            grid_contents: self.grid_contents.unwrap(),
        }
    }

    pub fn point_up(mut self) -> Self {
        self.point_up = true;
        self
    }

    pub fn with_dimensions(mut self, width: u32, height: u32) -> Self {
        self.dimensions = (width, height);
        self
    }
}

pub struct HexGrid {
    dimensions: (u32, u32),
    tile_size: u32,
    vbos: [fgl::VertexBuffer; 3],
    vao: fgl::VertexAttribObject,
    texture: fgl::texture::Texture2D,
    tilecount: u32,
    grid_contents: Vec<isize>,
}

impl Drop for HexGrid {
    fn drop(&mut self) {
        for buffer in self.vbos.iter() {
            buffer.delete()
        }
        // TODO: delete everything else
    }
}

impl HexGrid {
    pub unsafe fn draw(&self, program: &fgl::Program, projection: cgmath::Matrix4<f32>) {
        program.bind();
        program.uniform_mat4("projection", &projection);
        program.uniform_vec2(
            "size",
            [self.tile_size as f32, self.tile_size as f32].into(),
        );
        program.uniform_f32("ntiles", self.tilecount as f32);
        self.vao.bind();
        self.texture.bind(0);
        gl::DrawArraysInstanced(
            gl::TRIANGLES,
            0,
            6i32,
            (self.dimensions.0 * self.dimensions.1 * 6) as i32,
        );
    }

    pub fn update_tile(&mut self, idx: (usize, usize), tile: Option<usize>) {
        let idx = idx.1 + self.dimensions.1 as usize * idx.0;
        let tile = tile.map(|x| x as isize).unwrap_or(-1);
        self.vbos[2].map_data().put(idx, tile as f32);
        self.vbos[2].unmap_data();
        self.grid_contents[idx] = tile;
    }
}
