use bevy::prelude::Vec3;

enum VoxelType{
    Empty,
    Wire,
    AndGate,
    Input(bool),
    Output(bool),
}

struct Voxel {
    voxel_type: VoxelType,
    state: VoxelState,
    position: Vec3,
    // ...
}

enum VoxelState {
    On,
    Off,
}

type VoxelGrid = Vec<Vec<Vec<Option<Voxel>>>>;

impl Voxel {
    fn propagate_signal(&self, neighbors: &[Voxel]) -> VoxelState {
        todo!()
    }
}