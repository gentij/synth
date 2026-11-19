fn main() {
    platform_probe::enable_standalone_audition();
    truce_standalone::run::<platform_probe::Plugin>();
}
