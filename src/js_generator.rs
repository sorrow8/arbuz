use crate::predict_generator::generate_prediction;
use crate::roman_numerals::to_roman;
use serde_json::{Value, json};
use anyhow::Result;
use sha2::{Sha256, Digest};

const JS_TEMPLATES_JSON: &str = include_str!("js-templates.json");

pub struct JsGenerator;

impl JsGenerator {
  fn get_js_templates() -> Value {
    serde_json::from_str(JS_TEMPLATES_JSON).unwrap()
  }

  // Helper function to get mystical code based on card type
  fn get_mystical_code(encoded: u64, mystical_bits: u64, background_bits: u64, card_bits: u64, is_absolute: bool, is_glitch: bool, mystical_index: u64) -> usize {
    if is_absolute {
      // absolute offset = 1, 5, 9
      let offset = match mystical_index {
        0 => 1,
        1 => 5,
        2 => 9,
        _ => 1
      };
      ((encoded >> offset) & ((1u64 << mystical_bits) - 1)) as usize
    } else if is_glitch {
      // glitch
      let offset = match mystical_index {
        0 => background_bits + card_bits,
        1 => background_bits + card_bits + mystical_bits,
        2 => background_bits + card_bits + mystical_bits + mystical_bits,
        _ => background_bits + card_bits
      };
      ((encoded >> offset) & ((1u64 << mystical_bits) - 1)) as usize
    } else {
      // classic
      let offset = match mystical_index {
        0 => background_bits + card_bits,
        1 => background_bits + card_bits + mystical_bits,
        2 => background_bits + card_bits + mystical_bits + mystical_bits,
        _ => background_bits + card_bits
      };
      ((encoded >> offset) & ((1u64 << mystical_bits) - 1)) as usize
    }
  }

  // Helper function to get mystical symbols array
  fn get_mystical_symbols(mystical1_code: usize, mystical2_code: usize, mystical3_code: usize, symbols: &[&'static str]) -> Vec<&'static str> {
    let m1 = symbols[mystical1_code % symbols.len()];
    let m2 = symbols[mystical2_code % symbols.len()];
    let m3 = symbols[mystical3_code % symbols.len()];
    vec![m1, m2, m3]
  } 

  // Helper function to get template value with fallback
  fn get_template_value(templates: &Value, category: &str, key: &str, fallback: &str) -> String {
    templates[category][key].as_str().unwrap_or(fallback).to_string()
  }

  // Helper function to get card title values
  fn get_card_title_values(templates: &Value, card_title: &str) -> (String, String) {
    if let Some(title_array) = templates["cardTitles"][card_title].as_array() {
      (
        title_array[0].as_str().unwrap_or("").to_string(),
        title_array[1].as_str().unwrap_or("").to_string()
      )
    } else {
      let title_str = templates["cardTitles"][card_title].as_str().unwrap_or("").to_string();
      (title_str.clone(), title_str)
    }
  }

  // Helper function to check if card is special
  fn is_special_card(card_title: &str) -> bool {
    const SPECIAL_CARDS: [&str; 10] = [
      "airhead_card", "mist_card", "puppet_card", "taco_card", 
      "acai_card", "diesel_card", "clockin_card", "cheekyb_card", 
      "fartane_card", "arbuz_card"
    ];
    SPECIAL_CARDS.contains(&card_title)
  }

  pub fn decode_traits(index: u128) -> Result<(String, String, Vec<&'static str>, String, String, String)> {
    // Special case for index 0 - GENESIS card
    if index == 0 {
      return Ok((
        "ethereal_white".to_string(),
        "genesis".to_string(),
        vec!["genesis", "genesis", "genesis"],
        "genesis".to_string(),
        "gold".to_string(),
        "gold".to_string()
      ));
    }
    
    let backgrounds = vec!["mystical_purple", "cosmic_blue", "golden_mystic", "rose_gold", "dark_void", "emerald_green", "blood_red", "neon_pink", "cyber_yellow", "arctic_aqua", "lava_orange", "abyss_blue", "toxic_lime", "ethereal_white", "obsidian_black", "ultraviolet"];
    let border_colors = vec!["gold", "silver", "bronze", "purple", "blue", "red", "green"];
    let glow_colors = vec!["gold", "silver", "purple", "blue", "green", "red"];
    let classic_main_symbols = vec!["star", "moon", "sun", "tower", "wheel", "hermit", "magician", "priestess", "emperor", "empress", "devil", "fool", "hierophant", "lovers", "chariot", "strength", "justice", "hanged_man", "death", "temperance", "judgement", "world"];
    let classic_card_titles = vec!["the_star", "the_moon", "the_sun", "the_tower", "the_wheel", "the_hermit", "the_magician", "the_priestess", "the_emperor", "the_empress", "the_devil", "the_fool", "the_hierophant", "the_lovers", "the_chariot", "strength", "justice", "the_hanged_man", "death", "temperance", "judgement", "the_world"];
    let glitch_main_symbols = vec!["balloon", "flask", "puppet", "taco", "acai", "diesel", "clock", "chick"];
    let glitch_card_titles = vec!["airhead_card", "mist_card", "puppet_card", "taco_card", "acai_card", "diesel_card", "clockin_card", "cheekyb_card"];
    let absolute_main_symbol = vec!["fartane", "arbuz"];
    
    let mut hasher = Sha256::new();
    hasher.update(index.to_le_bytes());
    let hash = hasher.finalize();
    
    let encoded = u64::from_le_bytes(hash[0..8].try_into().unwrap());
    
    // Bit configuration constants
    const BACKGROUND_BITS: u64 = 4;
    const CLASSIC_CARD_BITS: u64 = 5;
    const GLITCH_CARD_BITS: u64 = 3;
    const MYSTICAL_BITS: u64 = 5;
    const BORDER_BITS: u64 = 3;
    const GLOW_BITS: u64 = 3;
    
    let background_code = (encoded & ((1u64 << BACKGROUND_BITS) - 1)) as usize;
    let absolute_chance_byte = hash[25];
    let is_absolute = absolute_chance_byte < 1;
    
    let chance_byte = hash[24];
    let is_glitch = !is_absolute && chance_byte < 13;
    
    let (main_symbol, card_title, border_color, glow_color) = if is_absolute {
      let absolute_card_code = (encoded & 1) as usize;
      if absolute_card_code == 0 {
        (
          "fartane",
          "fartane_card", 
          "silver",
          "silver"
        )
      } else {
        (
          "arbuz",
          "arbuz_card",
          "green",
          "green"
        )
      }
    } else if is_glitch {
      let glitch_card_code = ((encoded >> BACKGROUND_BITS) & ((1u64 << GLITCH_CARD_BITS) - 1)) as usize;
      (
        glitch_main_symbols[glitch_card_code % glitch_main_symbols.len()],
        glitch_card_titles[glitch_card_code % glitch_card_titles.len()],
        "gold",
        "gold"
      )
    } else {
      let card_code = ((encoded >> BACKGROUND_BITS) & ((1u64 << CLASSIC_CARD_BITS) - 1)) as usize;
      let border_code = ((encoded >> (BACKGROUND_BITS + CLASSIC_CARD_BITS + MYSTICAL_BITS + MYSTICAL_BITS + MYSTICAL_BITS)) & ((1u64 << BORDER_BITS) - 1)) as usize;
      let glow_code = ((encoded >> (BACKGROUND_BITS + CLASSIC_CARD_BITS + MYSTICAL_BITS + MYSTICAL_BITS + MYSTICAL_BITS + BORDER_BITS)) & ((1u64 << GLOW_BITS) - 1)) as usize;
      (
        classic_main_symbols[card_code % classic_main_symbols.len()],
        classic_card_titles[card_code % classic_card_titles.len()],
        border_colors[border_code % border_colors.len()],
        glow_colors[glow_code % glow_colors.len()]
      )
    };

    // Get mystical codes using helper function
    let mystical1_code = Self::get_mystical_code(encoded, MYSTICAL_BITS, BACKGROUND_BITS, if is_glitch { GLITCH_CARD_BITS } else { CLASSIC_CARD_BITS }, is_absolute, is_glitch, 0);
    let mystical2_code = Self::get_mystical_code(encoded, MYSTICAL_BITS, BACKGROUND_BITS, if is_glitch { GLITCH_CARD_BITS } else { CLASSIC_CARD_BITS }, is_absolute, is_glitch, 1);
    let mystical3_code = Self::get_mystical_code(encoded, MYSTICAL_BITS, BACKGROUND_BITS, if is_glitch { GLITCH_CARD_BITS } else { CLASSIC_CARD_BITS }, is_absolute, is_glitch, 2);
    
    // Get mystical symbols array using helper function
    let mystical_symbols_array = if is_absolute {
      Self::get_mystical_symbols(mystical1_code, mystical2_code, mystical3_code, &absolute_main_symbol)
    } else if is_glitch {
      Self::get_mystical_symbols(mystical1_code, mystical2_code, mystical3_code, &glitch_main_symbols)
    } else {
      Self::get_mystical_symbols(mystical1_code, mystical2_code, mystical3_code, &classic_main_symbols)
    };
    
    let background = if is_absolute {
      let absolute_card_code = (encoded & 1) as usize;
      if absolute_card_code == 0 {
        "lava_orange"
      } else {
        "blood_red"
      }
    } else {
      backgrounds[background_code % backgrounds.len()]
    };
    Ok((
      background.to_string(),
      main_symbol.to_string(),
      mystical_symbols_array,
      card_title.to_string(),
      border_color.to_string(),
      glow_color.to_string()
    ))
  }

  pub fn get_attributes(index: u128) -> Result<String> {
    let (background, main_symbol, mystical_symbols_array, card_title, border_color, glow_color) = Self::decode_traits(index)?;
    let (prediction, _prediction_cn) = crate::predict_generator::generate_prediction(index);

    let js_templates = Self::get_js_templates();

    let attributes = json!({
      "background": background,
      "mainSymbol": Self::get_template_value(&js_templates, "mainSymbol", &main_symbol, "💩"),
      "mysticalSymbols": mystical_symbols_array.iter()
        .map(|symbol| Self::get_template_value(&js_templates, "mainSymbol", symbol, "💩"))
        .collect::<Vec<String>>()
        .join(","),
      "cardTitle": card_title,
      "cardNumberIndex": index.to_string(),
      "borderColor": border_color,
      "glowColor": glow_color,
      "prediction": prediction
    });

    Ok(attributes.to_string())
  }

  pub fn generate_js(index: u128) -> Result<String> {
    let (background, main_symbol, mystical_symbols_array, card_title, border_color, glow_color) = Self::decode_traits(index)?;
    let (prediction_eng, prediction_cn) = generate_prediction(index);

    let index_display = if index == 0 {
        "GENESIS".to_string()
    } else {
        to_roman(index)
    };

    let js_templates = Self::get_js_templates();

    // Get template values using helper functions
    let background_value = Self::get_template_value(&js_templates, "background", &background, "linear-gradient(135deg,rgb(255, 255, 255) 0%,rgb(255, 255, 255) 50%,rgb(255, 255, 255) 100%)");
    let main_symbol_value = Self::get_template_value(&js_templates, "mainSymbol", &main_symbol, "");
    let border_color_value = Self::get_template_value(&js_templates, "borderColors", &border_color, "#ffffff");
    let glow_color_value = Self::get_template_value(&js_templates, "glowColors", &glow_color, "transparent");
    
    let is_special_card = Self::is_special_card(&card_title);
    let (card_title_value, card_title_cn_value) = Self::get_card_title_values(&js_templates, &card_title);

    let mut js = String::from("function createMagicArbuzCard(containerId) {\n  const container = document.getElementById(containerId);\n  if (!container) {\n    console.error('Container with id ' + containerId + ' not found');\n    return;\n  }\n\n  const cardData = {\n    title: '");
    js.push_str(&card_title_value);
    js.push_str("',\n    title_cn: '");
    js.push_str(&card_title_cn_value);
    js.push_str("',\n    isSpecialCard: ");
    js.push_str(&is_special_card.to_string());
    js.push_str(",\n    subtitle: '");
    js.push_str(&index_display);
    js.push_str("',\n    message_eng: '");
    js.push_str(&prediction_eng);
    js.push_str("',\n    message_cn: '");
    js.push_str(&prediction_cn);
    js.push_str("',\n    description: ''\n  };\n\n  const styles = `\n    <style>\n      @import url('https://fonts.googleapis.com/css2?family=Cormorant+Garamond:ital,wght@0,300;0,400;0,500;0,600;0,700;1,300;1,400;1,500;1,600;1,700&family=Noto+Serif+SC:wght@400;500;600;700&display=swap');\n      @keyframes twinkle {\n        0% { opacity: 0.7; transform: scale(1); filter: drop-shadow(0 0 10px ");
      js.push_str(&glow_color_value);
      js.push_str("); }\n        50% { opacity: 1; transform: scale(1.05); filter: drop-shadow(0 0 20px ");
      js.push_str(&glow_color_value);
      js.push_str("); }\n        100% { opacity: 0.8; transform: scale(1.02); filter: drop-shadow(0 0 15px ");
      js.push_str(&glow_color_value);
      js.push_str("); }\n      }\n      @keyframes sparkle {\n        0% { opacity: 0.3; transform: scale(0.8); filter: drop-shadow(0 0 5px rgba(199, 210, 254, 0.4)); }\n        50% { opacity: 1; transform: scale(1.2); filter: drop-shadow(0 0 10px rgba(199, 210, 254, 0.8)); }\n        100% { opacity: 0.5; transform: scale(0.9); filter: drop-shadow(0 0 7px rgba(199, 210, 254, 0.6)); }\n      }\n      @keyframes glow {\n        0% { opacity: 0.6; transform: scale(1); filter: drop-shadow(0 0 5px currentColor); }\n        50% { opacity: 1; transform: scale(1.04); filter: drop-shadow(0 0 15px currentColor); }\n        100% { opacity: 0.7; transform: scale(1.02); filter: drop-shadow(0 0 10px currentColor); }\n      }\n      @keyframes cardGlow {\n        0% { box-shadow: inset 0 0 20px ");
            js.push_str(&glow_color_value);
      js.push_str(", 0 0 30px ");
      js.push_str(&glow_color_value);
      js.push_str(", 0 0 15px ");
      js.push_str(&border_color_value);
      js.push_str("; border-color: ");
      js.push_str(&border_color_value);
      js.push_str("; }\n        50% { box-shadow: inset 0 0 30px ");
      js.push_str(&glow_color_value);
      js.push_str(", 0 0 50px ");
      js.push_str(&glow_color_value);
      js.push_str(", 0 0 25px ");
      js.push_str(&border_color_value);
      js.push_str("; border-color: ");
      js.push_str(&border_color_value);
      js.push_str("; }\n        100% { box-shadow: inset 0 0 25px ");
      js.push_str(&glow_color_value);
      js.push_str(", 0 0 40px ");
      js.push_str(&glow_color_value);
      js.push_str(", 0 0 20px ");
      js.push_str(&border_color_value);
      js.push_str("; border-color: ");
      js.push_str(&border_color_value);
      js.push_str("; }\n      }\n      @keyframes backgroundShimmer {\n        0% { opacity: 0.3; background: radial-gradient(circle at 30% 30%, ");
      js.push_str(&glow_color_value);
      js.push_str(" 0%, transparent 70%); }\n        25% { opacity: 0.6; background: radial-gradient(circle at 70% 40%, ");
      js.push_str(&glow_color_value);
      js.push_str(" 0%, transparent 70%); }\n        50% { opacity: 0.8; background: radial-gradient(circle at 50% 70%, ");
      js.push_str(&glow_color_value);
      js.push_str(" 0%, transparent 70%); }\n        75% { opacity: 0.4; background: radial-gradient(circle at 20% 60%, ");
      js.push_str(&glow_color_value);
      js.push_str(" 0%, transparent 70%); }\n        100% { opacity: 0.5; background: radial-gradient(circle at 80% 20%, ");
      js.push_str(&glow_color_value);
      js.push_str(" 0%, transparent 70%); }\n      }\n      .magic-arbuz-card-wrapper {\n        display: flex;\n        justify-content: center;\n        align-items: center;\n        perspective: 1000px;\n        min-height: 500px;\n      }\n      .magic-arbuz-card {\n        width: 400px;\n        height: 650px;\n        position: relative;\n        cursor: pointer;\n        transform-style: preserve-3d;\n        transition: all 0.1s ease-out;\n        transform: rotateY(0deg) rotateX(0deg) scale(1);\n        filter: drop-shadow(0 20px 40px rgba(0,0,0,0.5));\n        box-sizing: border-box;\n      }\n      .magic-arbuz-card-front {\n        position: absolute;\n        width: 100%;\n        height: 100%;\n        background: ");
    js.push_str(&background_value);
    js.push_str(";\n        border-radius: 12px;\n        border: 3px solid ");
    js.push_str(&border_color_value);
    js.push_str(";\n        box-shadow: inset 0 0 20px ");
    js.push_str(&glow_color_value);
    js.push_str(", 0 0 30px ");
    js.push_str(&glow_color_value);
    js.push_str(";\n        animation: cardGlow 4s ease-in-out infinite alternate;\n        display: flex;\n        flex-direction: column;\n        justify-content: space-between;\n        padding: 25px;\n        backface-visibility: hidden;\n        position: relative;\n        overflow: hidden;\n        box-sizing: border-box;\n      }\n      .mystical-background {\n        position: absolute;\n        top: 0;\n        left: 0;\n        right: 0;\n        bottom: 0;\n        background: radial-gradient(circle at 50% 50%, ");
      js.push_str(&glow_color_value);
      js.push_str(" 0%, transparent 70%);\n        opacity: 0.5;\n        animation: backgroundShimmer 5s ease-in-out infinite alternate;\n      }\n      .card-number {\n        text-align: center;\n        margin-bottom: 20px;\n      }\n      .card-number-text {\n        font-size: 28px;\n        font-weight: 700;\n        color: #ffd700;\n        text-shadow: -0.5px -0.5px 0 #222, 0.5px -0.5px 0 #222, -0.5px 0.5px 0 #222, 0.5px 0.5px 0 #222, 0 1px 4px rgba(30,30,30,0.18),0 0 10px ");
      js.push_str(&glow_color_value);
      js.push_str(";\n        font-family: serif;\n        letter-spacing: 2px;\n        animation: glow 3s ease-in-out infinite alternate;\n        animation-delay: 0.5s;\n        margin: 0;\n      }\n      .card-title-text {\n        font-size: 20px;\n        font-weight: 600;\n        color: #e0e7ff;\n        text-shadow: -0.5px -0.5px 0 #222, 0.5px -0.5px 0 #222, -0.5px 0.5px 0 #222, 0.5px 0.5px 0 #222, 0 0 8px rgba(224,231,255,0.5);\n        font-family: serif;\n        letter-spacing: 1px;\n        margin-top: 5px;\n        animation: glow 3.5s ease-in-out infinite alternate;\n        animation-delay: 0.8s;\n        margin-bottom: 0;\n        min-height: 30px;\n        display: flex;\n        align-items: center;\n        justify-content: center;\n      }\n      .central-illustration {\n        display: flex;\n        flex-direction: column;\n        align-items: center;\n        justify-content: center;\n        flex: 1;\n        position: relative;\n      }\n      .main-symbol {\n        font-size: 80px;\n        color: ");
    js.push_str(&border_color_value);
    js.push_str(";\n        text-shadow: 0 0 20px ");
    js.push_str(&glow_color_value);
    js.push_str(";\n        animation: twinkle 4s ease-in-out infinite alternate;\n        z-index: 2;\n        position: relative;\n      }\n      .mystical-symbols {\n        display: flex;\n        justify-content: center;\n        gap: 10px;\n        margin-bottom: 10px;\n        margin-top: 0;\n      }\n      .mystical-symbol {\n        font-size: 32px;\n        text-shadow: 0 0 8px #a78bfa;\n        animation: glow 2.5s ease-in-out infinite alternate;\n      }\n      .small-stars-orbit{position:absolute;left:50%;top:50%;transform:translate(-50%,-50%);width:120px;height:120px;pointer-events:none;}.small-stars{font-size:24px;color:#c7d2fe;animation:sparkle 3s ease-in-out infinite alternate;z-index:1;pointer-events:none;}\n      .card-message {\n        text-align: center;\n        margin-top: 20px;\n      }\n      .card-message-text {\n        font-size: 20px;\n        font-weight: 700;\n        color: #cbd5e1;\n        font-style: normal;\n        line-height: 1.3;\n        font-family: 'Cormorant Garamond', serif;\n        letter-spacing: 1px;\n        text-shadow: -0.5px -0.5px 0 #222, 0.5px -0.5px 0 #222, -0.5px 0.5px 0 #222, 0.5px 0.5px 0 #222, 0 1px 4px rgba(30,30,30,0.18),0 0 5px rgba(203, 213, 225, 0.3);\n        animation: glow 4s ease-in-out infinite alternate;\n        animation-delay: 1.2s;\n        margin: 0;\n        margin-bottom: 30px;\n        padding: 0 15px;\n        min-height: 60px;\n        height: 80px;\n        display: flex;\n        align-items: center;\n        justify-content: center;\n        text-align: center;\n        overflow: hidden;\n      }\n      .magic-arbuz-card:hover {\n        transform: rotateY(5deg) rotateX(5deg) scale(1.02);\n        filter: drop-shadow(0 25px 50px rgba(0,0,0,0.6));\n      }\n      .magic-arbuz-card:active {\n        transform: rotateY(10deg) rotateX(10deg) scale(0.98);\n      }\n    </style>\n  `;\n\n  const html = `\n    <div class=\"magic-arbuz-card-wrapper\">\n      <div class=\"magic-arbuz-card\">\n        <div class=\"magic-arbuz-card-front\">\n          <div class=\"magic-arbuz-card-border-decoration\" style=\"pointer-events:none;position:absolute;top:0;left:0;width:100%;height:100%;z-index:1;\"><svg viewBox='0 0 400 650' width='100%' height='100%' fill='none' xmlns='http://www.w3.org/2000/svg' style='display:block;'><rect x='14' y='12' width='372' height='626' rx='12' stroke='#ffe066' stroke-width='2.5' opacity='0.35'/></svg></div>\n          <div class=\"mystical-background\"></div>\n          <div class=\"card-number\">\n            <div class=\"card-number-text\">");
    js.push_str(&index_display);
    js.push_str("</div>\n            <div class=\"card-title-text\">");
    js.push_str(&card_title_value);
    js.push_str("</div>\n          </div>\n          <div class=\"central-illustration\">\n            <div class=\"main-star-container\" style=\"position:relative;margin-bottom:20px;\">\n              <div class=\"main-star\" style=\"font-size:140px;color:#ffd700;text-shadow:0 0 20px rgba(255,215,0,0.8);position:relative;z-index:2;filter:drop-shadow(0 0 10px rgba(255,215,0,0.5));animation:twinkle 2s ease-in-out infinite alternate;\">");
    js.push_str(&main_symbol_value);
    js.push_str("</div>");
    js.push_str("<div class=\"small-star small-star-1\" style=\"position:absolute;top:-20px;left:-30px;font-size:30px;color:#c7d2fe;text-shadow:0 0 10px rgba(199,210,254,0.6);z-index:1;animation:sparkle 1.5s ease-in-out infinite alternate;animation-delay:0.2s;animation-duration:1.5s;\">✦</div>");
    js.push_str("<div class=\"small-star small-star-2\" style=\"position:absolute;top:-15px;right:-25px;font-size:25px;color:#c7d2fe;text-shadow:0 0 10px rgba(199,210,254,0.6);z-index:1;animation:sparkle 1.5s ease-in-out infinite alternate;animation-delay:0.5s;animation-duration:1.8s;\">✦</div>");
    js.push_str("<div class=\"small-star small-star-3\" style=\"position:absolute;bottom:-20px;left:-20px;font-size:20px;color:#c7d2fe;text-shadow:0 0 10px rgba(199,210,254,0.6);z-index:1;animation:sparkle 1.5s ease-in-out infinite alternate;animation-delay:0.8s;animation-duration:2.2s;\">✦</div>");
    js.push_str("<div class=\"small-star small-star-4\" style=\"position:absolute;bottom:-15px;right:-30px;font-size:35px;color:#c7d2fe;text-shadow:0 0 10px rgba(199,210,254,0.6);z-index:1;animation:sparkle 1.5s ease-in-out infinite alternate;animation-delay:1.1s;animation-duration:1.3s;\">✦</div>");
    js.push_str("</div>\n");
    js.push_str("<div class=\"mystical-symbols\" style=\"display:flex;justify-content:center;gap:10px;margin-bottom:10px;margin-top:0;\">");
    for symbol in mystical_symbols_array {
      let mystical_symbol_value = js_templates["mainSymbol"][symbol].as_str().unwrap_or("💩");
      js.push_str("<div class=\"mystical-symbol\">");
      js.push_str(&mystical_symbol_value);
      js.push_str("</div>");
    }
    js.push_str("</div>\n");
    js.push_str("</div>\n          <div class=\"card-message\">\n            <div class=\"card-message-text\">");
    js.push_str(&prediction_eng);
    js.push_str("</div>\n          </div>\n");

    js.push_str("\n        </div>\n      </div>\n    </div>\n  `;\n\n  container.innerHTML = styles + html;\n\n  const magicArbuzCard = container.querySelector('.magic-arbuz-card');\n  if (magicArbuzCard) {\n    let lastRotateX = 0;\n    let lastRotateY = 0;\n    magicArbuzCard.addEventListener('mousemove', function(e) {\n      const rect = magicArbuzCard.getBoundingClientRect();\n      const x = e.clientX - rect.left;\n      const y = e.clientY - rect.top;\n      const centerX = rect.width / 2;\n      const centerY = rect.height / 2;\n      const deltaX = x - centerX; const deltaY = y - centerY; const distance = Math.sqrt(deltaX * deltaX + deltaY * deltaY); const maxDistance = Math.sqrt(centerX * centerX + centerY * centerY); const normalizedDistance = Math.min(distance / maxDistance, 1); const intensity = Math.pow(normalizedDistance, 0.6) * 1.2 + 0.4;\n      let rotateX = (deltaY / centerY) * 15 * intensity;\n      let rotateY = (-deltaX / centerX) * 15 * intensity;\n      const maxAngle = 15;\n      rotateX = Math.max(-maxAngle, Math.min(maxAngle, rotateX));\n      rotateY = Math.max(-maxAngle, Math.min(maxAngle, rotateY));\n      const smoothFactor = 0.15;\n      rotateX = lastRotateX + (rotateX - lastRotateX) * smoothFactor;\n      rotateY = lastRotateY + (rotateY - lastRotateY) * smoothFactor;\n      lastRotateX = rotateX; lastRotateY = rotateY;\n      magicArbuzCard.style.transform = 'rotateY(' + rotateY + 'deg) rotateX(' + rotateX + 'deg) scale(1.05)';\n      magicArbuzCard.style.filter = 'drop-shadow(0 30px 60px rgba(0,0,0,0.6))';\n    });\n    magicArbuzCard.addEventListener('mouseleave', function() {\n      lastRotateX = 0; lastRotateY = 0;\n      magicArbuzCard.style.transform = 'rotateY(0deg) rotateX(0deg) scale(1)';\n      magicArbuzCard.style.filter = 'drop-shadow(0 20px 40px rgba(0,0,0,0.5))';\n    });\n  }\n  \n  const messageDiv = container.querySelector('.card-message-text');\n  const titleDiv = container.querySelector('.card-title-text');\n  let currentLang = 'eng';\n  let clickStartTime = 0;\n  let isLongPress = false;\n  let longPressTimer = null;\n  \n  if (magicArbuzCard && messageDiv && titleDiv) {\n    magicArbuzCard.addEventListener('mousedown', function() {\n      clickStartTime = Date.now();\n      isLongPress = false;\n      longPressTimer = setTimeout(function() {\n        isLongPress = true;\n      }, 300);\n    });\n    \n    magicArbuzCard.addEventListener('mouseup', function() {\n      clearTimeout(longPressTimer);\n      const clickDuration = Date.now() - clickStartTime;\n      if (clickDuration < 300 && !isLongPress) {\n        if (currentLang === 'eng') {\n          messageDiv.textContent = cardData.message_cn;\n          messageDiv.style.fontFamily = '\\'Noto Serif SC\\', serif';\n          messageDiv.style.fontSize = '18px';\n          messageDiv.style.letterSpacing = '0.5px';\n          messageDiv.style.minHeight = '60px';\n          if (!cardData.isSpecialCard) {\n            titleDiv.textContent = cardData.title_cn;\n            titleDiv.style.fontFamily = '\\'Noto Serif SC\\', serif';\n            titleDiv.style.fontSize = '18px';\n            titleDiv.style.letterSpacing = '0.5px';\n          }\n          titleDiv.style.minHeight = '30px';\n          currentLang = 'cn';\n        } else {\n          messageDiv.textContent = cardData.message_eng;\n          messageDiv.style.fontFamily = '\\'Cormorant Garamond\\', serif';\n          messageDiv.style.fontSize = '20px';\n          messageDiv.style.letterSpacing = '1px';\n          messageDiv.style.minHeight = '60px';\n          titleDiv.textContent = cardData.title;\n          titleDiv.style.fontFamily = 'serif';\n          titleDiv.style.fontSize = '20px';\n          titleDiv.style.letterSpacing = '1px';\n          titleDiv.style.minHeight = '30px';\n          currentLang = 'eng';\n        }\n      }\n    });\n  }\n}\n\nif (typeof document !== 'undefined') {\n  document.addEventListener('DOMContentLoaded', function() {\n    createMagicArbuzCard('magic-arbuz-container');\n  });\n}");

    Ok(js)
  }
} 