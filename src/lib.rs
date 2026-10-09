use lopdf::{Dictionary, Document, Object};
use std::path::Path;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    R2L,
    L2R,
}

impl Mode {
    fn name(self) -> &'static str {
        match self {
            Mode::R2L => "R2L",
            Mode::L2R => "L2R",
        }
    }
}

/// ViewerPreferences の Direction を設定して保存する。
pub fn convert(src: &Path, dest: &Path, mode: Mode) -> Result<(), String> {
    let mut doc = Document::load(src).map_err(|e| format!("読み込み失敗: {e}"))?;
    let root_id = doc
        .trailer
        .get(b"Root")
        .and_then(|o| o.as_reference())
        .map_err(|e| format!("Catalogが見つかりません: {e}"))?;

    // ViewerPreferences が間接参照の場合は参照先を直接更新する
    let vp_ref = doc
        .get_dictionary(root_id)
        .ok()
        .and_then(|c| c.get(b"ViewerPreferences").ok())
        .and_then(|o| o.as_reference().ok());
    let dir = Object::Name(mode.name().as_bytes().to_vec());
    if let Some(id) = vp_ref {
        doc.get_dictionary_mut(id)
            .map_err(|e| e.to_string())?
            .set("Direction", dir);
    } else {
        let catalog = doc.get_dictionary_mut(root_id).map_err(|e| e.to_string())?;
        match catalog.get_mut(b"ViewerPreferences") {
            Ok(Object::Dictionary(d)) => d.set("Direction", dir),
            _ => {
                let mut d = Dictionary::new();
                d.set("Direction", dir);
                catalog.set("ViewerPreferences", Object::Dictionary(d));
            }
        }
    }
    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    doc.save(dest).map_err(|e| format!("保存失敗: {e}"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use lopdf::dictionary;

    fn sample(path: &Path) {
        let mut doc = Document::with_version("1.5");
        let pages = doc.new_object_id();
        let page = doc.add_object(dictionary! {"Type"=>"Page","Parent"=>pages,"MediaBox"=>vec![0.into(),0.into(),100.into(),100.into()]});
        doc.objects.insert(pages, Object::Dictionary(dictionary! {"Type"=>"Pages","Kids"=>vec![page.into()],"Count"=>1}));
        let cat = doc.add_object(dictionary! {"Type"=>"Catalog","Pages"=>pages});
        doc.trailer.set("Root", cat);
        doc.save(path).unwrap();
    }

    fn dir_of(path: &Path) -> Vec<u8> {
        let doc = Document::load(path).unwrap();
        let root = doc.trailer.get(b"Root").unwrap().as_reference().unwrap();
        let vp = doc.get_dictionary(root).unwrap().get(b"ViewerPreferences").unwrap().as_dict().unwrap();
        vp.get(b"Direction").unwrap().as_name().unwrap().to_vec()
    }

    #[test]
    fn sets_direction() {
        let d = std::env::temp_dir().join("pdfr2l_test");
        std::fs::create_dir_all(&d).unwrap();
        let (a, b, c) = (d.join("a.pdf"), d.join("b.pdf"), d.join("c.pdf"));
        sample(&a);
        convert(&a, &b, Mode::R2L).unwrap();
        assert_eq!(dir_of(&b), b"R2L");
        convert(&b, &c, Mode::L2R).unwrap();
        assert_eq!(dir_of(&c), b"L2R");
    }
}
