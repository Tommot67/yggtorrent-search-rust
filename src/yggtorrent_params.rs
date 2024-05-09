#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum YggCategory {
    All = 0,
    FilmVideo = 2145,
    Audio = 2139,
    Application = 2144,
    Jeux = 2142,
    Ebook = 2140,
    Print3D = 2200,
    Emulation = 2141,
    GPS = 2143,
}

impl YggCategory {
    pub(crate) fn get_base_url(&self) -> &str {
        let value = self.discriminant();
        if value == 0 {
            "&category=all"
        }
        else {
            let base_url = format!("&category={}", value);
            Box::leak(base_url.into_boxed_str())
        }
    }

    fn discriminant(&self) -> u32 {
        unsafe { *(self as *const YggCategory as *const u32) }
    }

    #[warn(unused_assignments)]
    pub(crate) fn have_this_subcategory(&self, subcategory: &YggSubCategory) -> bool {
        let mut min: u32 = 0;
        let mut max: u32 = 0;

        match self {
            YggCategory::FilmVideo => {
                min = 2178;
                max = 2187;
            },
            YggCategory::Audio => {
                min = 2147;
                max = 2150;
            },
            YggCategory::Application => {
                min = 2147;
                max = 2150;
            },
            YggCategory::Jeux => {
                min = 2159;
                max = 2167;
            },
            YggCategory::Ebook => {
                min = 2151;
                max = 2156;
            },
            YggCategory::Print3D => {
                min = 2201;
                max = 2202;
            }
            YggCategory::Emulation => {
                min = 2157;
                max = 2158;
            },
            YggCategory::GPS => {
                min = 2168;
                max = 2170;
            }
            _ => {
                min = 0;
                max = 0;
            }
        }

        (min <= YggSubCategory::discriminant(&subcategory) && YggSubCategory::discriminant(&subcategory) <= max) || YggSubCategory::discriminant(&subcategory) == 0
    }

}

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum YggSubCategory {
    All = 0,
    //FilmVideo (between 2178 and 2187) -> 7
    Animation = 2178,
    AnimationSerie = 2179,
    Concert = 2180,
    Documentaire = 2181,
    EmissionTV = 2182,
    Film = 2183,
    SerieTV = 2184,
    Spectacle = 2185,
    Sport = 2186,
    VideoClips = 2187,
    //Audio (between 2147 and 2150) -> 1
    Karaoke = 2147,
    Musique = 2148,
    PodcastRadio = 2150,
    Samples = 2149,
    //Application (between 2171 and 2177) -> 6
    AutreApplication = 2177,
    Formation = 2176,
    ApplicationLinux = 2171,
    ApplicationMacOS = 2172,
    ApplicationSmartphone = 2174,
    ApplicationTablette = 2175,
    ApplicationWindows = 2173,
    //Jeux (between 2159 and 2167) -> 4
    AutreJeux = 2167,
    JeuxLinux = 2159,
    JeuxMacOS = 2160,
    JeuxMicrosoft = 2162,
    JeuxNintendo = 2163,
    JeuxSmartphone = 2165,
    JeuxSony = 2164,
    JeuxTablette = 2166,
    JeuxWindows = 2161,
    //Ebook (between 2151 and 2156) -> 2
    Audio = 2151,
    Bds = 2152,
    Comics = 2153,
    Livres = 2154,
    Mangas = 2155,
    Presse = 2156,
    //Print3D (between 2201 and 2202)
    Objets = 2201,
    Personnages = 2022,
    //Emulation (between 2157 and 2158) -> 3
    Emulateurs = 2157,
    Roms = 2158,
    //GPS (between 2168 and 2170) -> 5
    Applications = 2168,
    Cartes = 2169,
    Divers = 2170,
}

impl YggSubCategory {
    pub(crate) fn get_base_url(&self) -> &str {
        let value = self.discriminant();
        if value == 0 {
            "&subcategory=all"
        }
        else {
            let base_url = format!("&subcategory={}", value);
            Box::leak(base_url.into_boxed_str())
        }
    }

    fn discriminant(&self) -> u32 {
        unsafe { *(self as *const YggSubCategory as *const u32) }
    }

    #[allow(dead_code)]
    pub(crate) fn is_in_this_category(&self, category: YggCategory) -> bool {
        category.have_this_subcategory(self)
    }

}

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum YggOrderElement {
    Commentaire = 0,
    DatePublication = 1,
    Taille = 2,
    TelechargementComplet = 3,
    Seed = 4,
    Leech = 5
}

impl YggOrderElement {
    pub(crate) fn get_base_url(&self) -> &str {
        let value = self.discriminant();
        match value {
            0 => "&sort=comments",
            1 => "&sort=publish_date",
            2 => "&sort=size",
            3 => "&sort=completed",
            4 => "&sort=seed",
            5 => "&sort=leech",
            _ => ""
        }
    }

    fn discriminant(&self) -> u32 {
        unsafe { *(self as *const YggOrderElement as *const u32) }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum YggOrder {
    Ascendant = 0,
    Descendant = 1
}

impl YggOrder {
    pub(crate) fn get_base_url(&self) -> &str {
        let value = self.discriminant();
        match value {
            0 => "&order=asc",
            1 => "&order=desc",
            _ => ""
        }
    }

    fn discriminant(&self) -> u32 {
        unsafe { *(self as *const YggOrder as *const u32) }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct YggParams {
    pub category: YggCategory,
    pub subcategory: YggSubCategory,
    pub uploader: Option<&'static str>,
    pub order: Option<(YggOrder,YggOrderElement)>
}

impl Default for YggParams {
    fn default() -> YggParams {
        YggParams { category: YggCategory::All , subcategory: YggSubCategory::All , uploader: None , order: None}
    }
}

impl YggParams {
    pub fn get_options_url(&self) -> &str {
        if self.category.have_this_subcategory(&self.subcategory) {
            let base_url = format!("{}{}{}{}", self.category.get_base_url(), self.subcategory.get_base_url() , self.get_uploader_base_url(), self.get_order_base_url());
            Box::leak(base_url.into_boxed_str())
        }
        else {
            let base_url = format!("{}{}{}{}", self.category.get_base_url(), YggSubCategory::All.get_base_url(), self.get_uploader_base_url(), self.get_order_base_url());
            Box::leak(base_url.into_boxed_str())
        }
    }

    pub fn concat_to_url(&self, base_url: &str) -> &str {
        let base_url = format!("{}{}", base_url, self.get_options_url());
        Box::leak(base_url.into_boxed_str())
    }

    fn get_uploader_base_url(&self) -> &str {
        if self.uploader.is_none() {
            "&uploader="
        }
        else {
            let base_url = format!("&uploader={}", self.uploader.unwrap());
            Box::leak(base_url.into_boxed_str())
        }
    }

    fn get_order_base_url(&self) -> &str {
        if self.order.is_some() {
            let base_url = format!("{}{}", self.order.unwrap().0.get_base_url() , self.order.unwrap().1.get_base_url());
            Box::leak(base_url.into_boxed_str())
        }
        else {
            ""
        }
    }

}