pub struct Patch {
    pub text_to_find: String,
    pub replacement_text: String,
}

pub fn get_patches() -> Vec<Patch> {
    vec![
        // Max TDP = 30
        Patch {
            text_to_find: "return[n,t,r,e=>i((()=>p.Get().SetTDPLimit(e)))".to_string(),
            replacement_text: "return[n,t,30,e=>i((()=>p.Get().SetTDPLimit(e)))".to_string(),
        },
        // Listen to TDP changes
        Patch {
            text_to_find: "const t=c.Hm.deserializeBinary(e).toObject();Object.keys(t)".to_string(),
            replacement_text: "const t=c.Hm.deserializeBinary(e).toObject(); fetch(`http://localhost:1338/set_tdp/${t.settings.per_app.tdp_limit}`); Object.keys(t)".to_string(),
        },
        // Add more patches as needed
    ]
}