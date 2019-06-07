
/// This is animal species,
/// or basically settings for a
/// collection of animals.
/// 
/// This defines properties of animals,
/// such as whether it's easy or hard
/// for them to walk (in terms of energy cost),
/// maximum brain size, or the energy gains
/// per food pellet.
pub struct Species {
    /// Energy per eaten food pellet.
    pub food_energy: isize,
    /// Energy per triggered Go action.
    /// (this is expected to be negative)
    pub go_energy: isize,
    /// Energy per triggered Eat action.
    /// (this is expected to be negative)
    pub eat_energy: isize,
}