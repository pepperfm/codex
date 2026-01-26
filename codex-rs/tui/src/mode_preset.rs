#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum ModePreset {
    #[default]
    Base,
    Soft,
    Strict,
    Nuxt,
    SoftNuxt,
    StrictNuxt,
}

impl ModePreset {
    pub(crate) const fn all() -> [ModePreset; 6] {
        [
            ModePreset::Base,
            ModePreset::Soft,
            ModePreset::Strict,
            ModePreset::Nuxt,
            ModePreset::SoftNuxt,
            ModePreset::StrictNuxt,
        ]
    }

    pub(crate) const fn label(self) -> &'static str {
        match self {
            ModePreset::Base => "Base",
            ModePreset::Soft => "Soft",
            ModePreset::Strict => "Strict",
            ModePreset::Nuxt => "Nuxt",
            ModePreset::SoftNuxt => "Soft + Nuxt",
            ModePreset::StrictNuxt => "Strict + Nuxt",
        }
    }

    pub(crate) const fn description(self) -> &'static str {
        match self {
            ModePreset::Base => "default / no prefix",
            ModePreset::Soft => "minimal diffs",
            ModePreset::Strict => "senior mode, patterns + trade-offs",
            ModePreset::Nuxt => "frontend Nuxt UI mode",
            ModePreset::SoftNuxt => "fullstack soft",
            ModePreset::StrictNuxt => "fullstack strict",
        }
    }

    pub(crate) const fn prefix(self) -> &'static str {
        match self {
            ModePreset::Base => "",
            ModePreset::Soft => "^soft ",
            ModePreset::Strict => "^strict ",
            ModePreset::Nuxt => "^nuxt ",
            ModePreset::SoftNuxt => "^soft ^nuxt ",
            ModePreset::StrictNuxt => "^strict ^nuxt ",
        }
    }
}
