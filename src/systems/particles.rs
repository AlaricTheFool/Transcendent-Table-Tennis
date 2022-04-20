use crate::prelude::*;

#[system(for_each)]
pub fn create_particles(entity: &Entity, p_burst: &CreateParticleBurstMessage, commands: &mut CommandBuffer) {
    spawn_particle_burst(commands, p_burst.position);

    commands.remove(*entity);
}

#[system(for_each)]
pub fn move_particles(particle: &Particle, pos: &mut Vec2) {
    let move_vec = particle.dir * 3.0;
    *pos += move_vec;
}

#[system(for_each)]
pub fn clear_particles(entity: &Entity, _: &Particle, commands: &mut CommandBuffer) {
    commands.remove(*entity);
}
