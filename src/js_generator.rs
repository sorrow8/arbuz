use crate::predict_generator::generate_prediction;
use serde_json::{Value, json};
use anyhow::Result;
use sha2::{Sha256, Digest};

const JS_TEMPLATES_JSON: &str = include_str!("js-templates.json");

pub struct JsGenerator;

impl JsGenerator {
  fn get_js_templates() -> Value {
    serde_json::from_str(JS_TEMPLATES_JSON).unwrap()
  }

  pub fn decode_traits(index: u128) -> Result<(String, String, Vec<&'static str>, String, String, String)> {
    let backgrounds = vec!["mystical_purple", "cosmic_blue", "golden_mystic", "rose_gold", "dark_void", "emerald_green", "blood_red", "neon_pink", "cyber_yellow", "arctic_aqua", "lava_orange", "abyss_blue", "toxic_lime", "ethereal_white", "obsidian_black", "ultraviolet"];
    let border_colors = vec!["gold", "silver", "bronze", "purple", "blue", "red", "green"];
    let glow_colors = vec!["gold", "silver", "purple", "blue", "green", "red"];
    let classic_main_symbols = vec!["star", "moon", "sun", "tower", "wheel", "hermit", "magician", "priestess", "emperor", "empress", "devil", "fool", "hierophant", "lovers", "chariot", "strength", "justice", "hanged_man", "death", "temperance", "judgement", "world"];
    let classic_card_titles = vec!["the_star", "the_moon", "the_sun", "the_tower", "the_wheel", "the_hermit", "the_magician", "the_priestess", "the_emperor", "the_empress", "the_devil", "the_fool", "the_hierophant", "the_lovers", "the_chariot", "strength", "the_justice", "the_hanged_man", "death", "temperance", "judgement", "the_world"];
    let glitch_main_symbols = vec!["balloon", "flask", "puppet", "taco", "acai", "diesel", "clock", "chick"];
    let glitch_card_titles = vec!["airhead_card", "mist_card", "puppet_card", "taco_card", "acai_card", "diesel_card", "clockin_card", "cheekyb_card"];
    let absolute_main_symbol = vec!["fartane", "arbuz"];
    
    let mut hasher = Sha256::new();
    hasher.update(index.to_le_bytes());
    let hash = hasher.finalize();
    
    let encoded = u64::from_le_bytes(hash[0..8].try_into().unwrap());
    
    let background_bits = 4;
    let classic_card_bits = 5;
    let glitch_card_bits = 3;
    let mystical_bits = 5;
    let border_bits = 3;
    let glow_bits = 3;
    
    let background_code = (encoded & ((1u64 << background_bits) - 1)) as usize;
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
      let glitch_card_code = ((encoded >> background_bits) & ((1u64 << glitch_card_bits) - 1)) as usize;
      (
        glitch_main_symbols[glitch_card_code % glitch_main_symbols.len()],
        glitch_card_titles[glitch_card_code % glitch_card_titles.len()],
        "gold",
        "gold"
      )
    } else {
      let card_code = ((encoded >> background_bits) & ((1u64 << classic_card_bits) - 1)) as usize;
      let border_code = ((encoded >> (background_bits + classic_card_bits + mystical_bits + mystical_bits + mystical_bits)) & ((1u64 << border_bits) - 1)) as usize;
      let glow_code = ((encoded >> (background_bits + classic_card_bits + mystical_bits + mystical_bits + mystical_bits + border_bits)) & ((1u64 << glow_bits) - 1)) as usize;
      (
        classic_main_symbols[card_code % classic_main_symbols.len()],
        classic_card_titles[card_code % classic_card_titles.len()],
        border_colors[border_code % border_colors.len()],
        glow_colors[glow_code % glow_colors.len()]
      )
    };

    let mystical1_code = if is_absolute {
      ((encoded >> 1) & ((1u64 << mystical_bits) - 1)) as usize
    } else if is_glitch {
      ((encoded >> (background_bits + glitch_card_bits)) & ((1u64 << mystical_bits) - 1)) as usize
    } else {
      ((encoded >> (background_bits + classic_card_bits)) & ((1u64 << mystical_bits) - 1)) as usize
    };
    let mystical2_code = if is_absolute {
      ((encoded >> 5) & ((1u64 << mystical_bits) - 1)) as usize
    } else if is_glitch {
      ((encoded >> (background_bits + glitch_card_bits + mystical_bits)) & ((1u64 << mystical_bits) - 1)) as usize
    } else {
      ((encoded >> (background_bits + classic_card_bits + mystical_bits)) & ((1u64 << mystical_bits) - 1)) as usize
    };
    let mystical3_code = if is_absolute {
      ((encoded >> 9) & ((1u64 << mystical_bits) - 1)) as usize
    } else if is_glitch {
      ((encoded >> (background_bits + glitch_card_bits + mystical_bits + mystical_bits)) & ((1u64 << mystical_bits) - 1)) as usize
    } else {
      ((encoded >> (background_bits + classic_card_bits + mystical_bits + mystical_bits)) & ((1u64 << mystical_bits) - 1)) as usize
    };
    
    let mystical_symbols_array = if is_absolute {
      let m1 = absolute_main_symbol[mystical1_code % absolute_main_symbol.len()];
      let m2 = absolute_main_symbol[mystical2_code % absolute_main_symbol.len()];
      let m3 = absolute_main_symbol[mystical3_code % absolute_main_symbol.len()];
      vec![m1, m2, m3]
    } else if is_glitch {
      let m1 = glitch_main_symbols[mystical1_code % glitch_main_symbols.len()];
      let m2 = glitch_main_symbols[mystical2_code % glitch_main_symbols.len()];
      let m3 = glitch_main_symbols[mystical3_code % glitch_main_symbols.len()];
      vec![m1, m2, m3]
    } else {
      let m1 = classic_main_symbols[mystical1_code % classic_main_symbols.len()];
      let m2 = classic_main_symbols[mystical2_code % classic_main_symbols.len()];
      let m3 = classic_main_symbols[mystical3_code % classic_main_symbols.len()];
      vec![m1, m2, m3]
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
    println!("[DEBUG] get_attributes called with index: {}", index);
    let (background, main_symbol, mystical_symbols_array, card_title, border_color, glow_color) = Self::decode_traits(index)?;
    let (prediction_eng, prediction_cn) = crate::predict_generator::generate_prediction(index);

    let attributes = json!({
      "background": background,
      "mainSymbol": main_symbol,
      "mysticalSymbols": mystical_symbols_array.join(","),
      "cardTitle": card_title,
      "cardNumberIndex": index.to_string(),
      "borderColor": border_color,
      "glowColor": glow_color,
      "prediction_en": prediction_eng,
      "prediction_cn": prediction_cn
    });

    Ok(attributes.to_string())
  }

  pub fn generate_js(index: u128) -> Result<String> {
    let (background, main_symbol, mystical_symbols_array, card_title, border_color, glow_color) = Self::decode_traits(index)?;
    let (prediction_eng, prediction_cn) = generate_prediction(index);

    let js_templates = Self::get_js_templates();

    let background_value = js_templates["background"][&background].as_str().unwrap_or("linear-gradient(135deg,rgb(255, 255, 255) 0%,rgb(255, 255, 255) 50%,rgb(255, 255, 255) 100%)");
    let main_symbol_value = js_templates["mainSymbol"][&main_symbol].as_str().unwrap_or("🤡");
    let card_title_value = js_templates["cardTitles"][&card_title].as_str().unwrap_or("THE FOOL");
    let border_color_value = js_templates["borderColors"][&border_color].as_str().unwrap_or("#ffffff");
    let glow_color_value = js_templates["glowColors"][&glow_color].as_str().unwrap_or("transparent");

    let mut js = String::from("function createTarotCard(containerId) {\n  const container = document.getElementById(containerId);\n  if (!container) {\n    console.error('Container with id ' + containerId + ' not found');\n    return;\n  }\n\n  const cardData = {\n    title: '");
    js.push_str(card_title_value);
    js.push_str("',\n    subtitle: '");
    js.push_str(&index.to_string());
    js.push_str("',\n    message_eng: '");
    js.push_str(&prediction_eng);
    js.push_str("',\n    message_cn: '");
    js.push_str(&prediction_cn);
    js.push_str("',\n    description: ''\n  };\n\n  const styles = `\n    <style>\n      @keyframes twinkle {\n        0% { opacity: 0.7; transform: scale(1); filter: drop-shadow(0 0 10px ");
      js.push_str(glow_color_value);
      js.push_str("); }\n        50% { opacity: 1; transform: scale(1.05); filter: drop-shadow(0 0 20px ");
      js.push_str(glow_color_value);
      js.push_str("); }\n        100% { opacity: 0.8; transform: scale(1.02); filter: drop-shadow(0 0 15px ");
      js.push_str(glow_color_value);
      js.push_str("); }\n      }\n      @keyframes sparkle {\n        0% { opacity: 0.3; transform: scale(0.8); filter: drop-shadow(0 0 5px rgba(199, 210, 254, 0.4)); }\n        50% { opacity: 1; transform: scale(1.2); filter: drop-shadow(0 0 10px rgba(199, 210, 254, 0.8)); }\n        100% { opacity: 0.5; transform: scale(0.9); filter: drop-shadow(0 0 7px rgba(199, 210, 254, 0.6)); }\n      }\n      @keyframes glow {\n        0% { opacity: 0.6; transform: scale(1); filter: drop-shadow(0 0 5px currentColor); }\n        50% { opacity: 1; transform: scale(1.1); filter: drop-shadow(0 0 15px currentColor); }\n        100% { opacity: 0.7; transform: scale(1.05); filter: drop-shadow(0 0 10px currentColor); }\n      }\n      @keyframes cardGlow {\n        0% { box-shadow: inset 0 0 20px ");
      js.push_str(glow_color_value);
      js.push_str(", 0 0 30px ");
      js.push_str(glow_color_value);
      js.push_str("; border-color: ");
      js.push_str(border_color_value);
      js.push_str("; }\n        50% { box-shadow: inset 0 0 30px ");
      js.push_str(glow_color_value);
      js.push_str(", 0 0 50px ");
      js.push_str(glow_color_value);
      js.push_str("; border-color: ");
      js.push_str(border_color_value);
      js.push_str("; }\n        100% { box-shadow: inset 0 0 25px ");
      js.push_str(glow_color_value);
      js.push_str(", 0 0 40px ");
      js.push_str(glow_color_value);
      js.push_str("; border-color: ");
      js.push_str(border_color_value);
      js.push_str("; }\n      }\n      @keyframes backgroundShimmer {\n        0% { opacity: 0.3; background: radial-gradient(circle at 30% 30%, ");
      js.push_str(glow_color_value);
      js.push_str(" 0%, transparent 70%); }\n        25% { opacity: 0.6; background: radial-gradient(circle at 70% 40%, ");
      js.push_str(glow_color_value);
      js.push_str(" 0%, transparent 70%); }\n        50% { opacity: 0.8; background: radial-gradient(circle at 50% 70%, ");
      js.push_str(glow_color_value);
      js.push_str(" 0%, transparent 70%); }\n        75% { opacity: 0.4; background: radial-gradient(circle at 20% 60%, ");
      js.push_str(glow_color_value);
      js.push_str(" 0%, transparent 70%); }\n        100% { opacity: 0.5; background: radial-gradient(circle at 80% 20%, ");
      js.push_str(glow_color_value);
      js.push_str(" 0%, transparent 70%); }\n      }\n      .tarot-card-wrapper {\n        display: flex;\n        justify-content: center;\n        align-items: center;\n        perspective: 1000px;\n        min-height: 500px;\n      }\n      .tarot-card {\n        width: 350px;\n        height: 600px;\n        position: relative;\n        cursor: pointer;\n        transform-style: preserve-3d;\n        transition: all 0.1s ease-out;\n        transform: rotateY(0deg) rotateX(0deg) scale(1);\n        filter: drop-shadow(0 20px 40px rgba(0,0,0,0.5));\n      }\n      .tarot-card-front {\n        position: absolute;\n        width: 100%;\n        height: 100%;\n        background: ");
    js.push_str(background_value);
          js.push_str(";\n        border-radius: 12px;\n        border: 3px solid ");
      js.push_str(border_color_value);
      js.push_str(";\n        box-shadow: inset 0 0 20px ");
      js.push_str(glow_color_value);
      js.push_str(", 0 0 30px ");
      js.push_str(glow_color_value);
      js.push_str(";\n        animation: cardGlow 4s ease-in-out infinite alternate;\n        display: flex;\n        flex-direction: column;\n        justify-content: space-between;\n        padding: 25px;\n        backface-visibility: hidden;\n        position: relative;\n        overflow: hidden;\n      }\n      .mystical-background {\n        position: absolute;\n        top: 0;\n        left: 0;\n        right: 0;\n        bottom: 0;\n        background: radial-gradient(circle at 50% 50%, ");
      js.push_str(glow_color_value);
      js.push_str(" 0%, transparent 70%);\n        opacity: 0.5;\n        animation: backgroundShimmer 5s ease-in-out infinite alternate;\n      }\n      .card-number {\n        text-align: center;\n        margin-bottom: 20px;\n      }\n      .card-number-text {\n        font-size: 28px;\n        font-weight: 700;\n        color: #ffd700;\n        text-shadow: -0.5px -0.5px 0 #222, 0.5px -0.5px 0 #222, -0.5px 0.5px 0 #222, 0.5px 0.5px 0 #222, 0 1px 4px rgba(30,30,30,0.18),0 0 10px ");
      js.push_str(glow_color_value);
      js.push_str(";\n        font-family: serif;\n        letter-spacing: 2px;\n        animation: glow 3s ease-in-out infinite alternate;\n        animation-delay: 0.5s;\n        margin: 0;\n      }\n      .card-title-text {\n        font-size: 20px;\n        font-weight: 600;\n        color: #e0e7ff;\n        text-shadow: -0.5px -0.5px 0 #222, 0.5px -0.5px 0 #222, -0.5px 0.5px 0 #222, 0.5px 0.5px 0 #222, 0 0 8px rgba(224,231,255,0.5);\n        font-family: serif;\n        letter-spacing: 1px;\n        margin-top: 5px;\n        animation: glow 3.5s ease-in-out infinite alternate;\n        animation-delay: 0.8s;\n        margin-bottom: 0;\n      }\n      .central-illustration {\n        display: flex;\n        flex-direction: column;\n        align-items: center;\n        justify-content: center;\n        flex: 1;\n        position: relative;\n      }\n      .main-symbol {\n        font-size: 80px;\n        color: ");
      js.push_str(border_color_value);
      js.push_str(";\n        text-shadow: 0 0 20px ");
      js.push_str(glow_color_value);
      js.push_str(";\n        animation: twinkle 4s ease-in-out infinite alternate;\n        z-index: 2;\n        position: relative;\n      }\n      .mystical-symbols {\n        display: flex;\n        justify-content: center;\n        gap: 10px;\n        margin-bottom: 10px;\n        margin-top: 0;\n      }\n      .mystical-symbol {\n        font-size: 32px;\n        text-shadow: 0 0 8px #a78bfa;\n        animation: glow 2.5s ease-in-out infinite alternate;\n      }\n      .small-stars-orbit{position:absolute;left:50%;top:50%;transform:translate(-50%,-50%);width:120px;height:120px;pointer-events:none;}.small-stars{font-size:24px;color:#c7d2fe;animation:sparkle 3s ease-in-out infinite alternate;z-index:1;pointer-events:none;}\n      .card-message {\n        text-align: center;\n        margin-top: 20px;\n      }\n      .card-message-text {\n        font-size: 16px;\n        font-weight: 600;\n        color: #cbd5e1;\n        font-style: italic;\n        line-height: 1.4;\n        text-shadow: -0.5px -0.5px 0 #222, 0.5px -0.5px 0 #222, -0.5px 0.5px 0 #222, 0.5px 0.5px 0 #222, 0 1px 4px rgba(30,30,30,0.18),0 0 5px rgba(203, 213, 225, 0.3);\n        animation: glow 4s ease-in-out infinite alternate;\n        animation-delay: 1.2s;\n        margin: 0;\n        margin-bottom: 40px;\n      }\n      .tarot-card:hover {\n        transform: rotateY(5deg) rotateX(5deg) scale(1.02);\n        filter: drop-shadow(0 25px 50px rgba(0,0,0,0.6));\n      }\n      .tarot-card:active {\n        transform: rotateY(10deg) rotateX(10deg) scale(0.98);\n      }\n    </style>\n  `;\n\n  const html = `\n    <div class=\"tarot-card-wrapper\">\n      <div class=\"tarot-card\">\n        <div class=\"tarot-card-front\">\n          <div class=\"tarot-card-border-decoration\" style=\"pointer-events:none;position:absolute;top:0;left:0;width:100%;height:100%;z-index:1;\"><svg viewBox='0 0 350 600' width='100%' height='100%' fill='none' xmlns='http://www.w3.org/2000/svg' style='display:block;'><rect x='6' y='6' width='338' height='588' rx='18' stroke='#222' stroke-width='2.5' opacity='0.18'/></svg></div>\n          <div class=\"mystical-background\"></div>\n          <div class=\"card-number\">\n            <div class=\"card-number-text\">");
    js.push_str(&index.to_string());
    js.push_str("</div>\n            <div class=\"card-title-text\">");
    js.push_str(card_title_value);
    js.push_str("</div>\n          </div>\n          <div class=\"central-illustration\">\n            <div class=\"main-star-container\" style=\"position:relative;margin-bottom:20px;\">\n              <div class=\"main-star\" style=\"font-size:140px;color:#ffd700;text-shadow:0 0 20px rgba(255,215,0,0.8);position:relative;z-index:2;filter:drop-shadow(0 0 10px rgba(255,215,0,0.5));animation:twinkle 2s ease-in-out infinite alternate;\">");
    js.push_str(main_symbol_value);
    js.push_str("</div>");
    js.push_str("<div class=\"small-star small-star-1\" style=\"position:absolute;top:-20px;left:-30px;font-size:30px;color:#c7d2fe;text-shadow:0 0 10px rgba(199,210,254,0.6);z-index:1;animation:sparkle 1.5s ease-in-out infinite alternate;animation-delay:0.2s;animation-duration:1.5s;\">✦</div>");
    js.push_str("<div class=\"small-star small-star-2\" style=\"position:absolute;top:-15px;right:-25px;font-size:25px;color:#c7d2fe;text-shadow:0 0 10px rgba(199,210,254,0.6);z-index:1;animation:sparkle 1.5s ease-in-out infinite alternate;animation-delay:0.5s;animation-duration:1.8s;\">✦</div>");
    js.push_str("<div class=\"small-star small-star-3\" style=\"position:absolute;bottom:-20px;left:-20px;font-size:20px;color:#c7d2fe;text-shadow:0 0 10px rgba(199,210,254,0.6);z-index:1;animation:sparkle 1.5s ease-in-out infinite alternate;animation-delay:0.8s;animation-duration:2.2s;\">✦</div>");
    js.push_str("<div class=\"small-star small-star-4\" style=\"position:absolute;bottom:-15px;right:-30px;font-size:35px;color:#c7d2fe;text-shadow:0 0 10px rgba(199,210,254,0.6);z-index:1;animation:sparkle 1.5s ease-in-out infinite alternate;animation-delay:1.1s;animation-duration:1.3s;\">✦</div>");
    js.push_str("</div>\n");
    js.push_str("<div class=\"mystical-symbols\" style=\"display:flex;justify-content:center;gap:10px;margin-bottom:10px;margin-top:0;\">");
    for symbol in mystical_symbols_array {
      let mystical_symbol_value = js_templates["mainSymbol"][symbol].as_str().unwrap_or("🤡");
      js.push_str("<div class=\"mystical-symbol\">");
      js.push_str(mystical_symbol_value);
      js.push_str("</div>");
    }
    js.push_str("</div>\n");
    js.push_str("</div>\n          <div class=\"card-message\">\n            <div class=\"card-message-text\">");
    js.push_str(&prediction_eng);
    js.push_str("</div>\n          </div>\n");
    js.push_str("<button class=\"lang-switch\" style=\"position:absolute;top:10px;right:10px;z-index:10;padding:4px 12px;border-radius:8px;border:none;background:#fff3;backdrop-filter:blur(2px);color:#6b7280;font-weight:700;cursor:pointer;transition:background 0.2s;\">🇨🇳</button>");
    js.push_str("\n        </div>\n      </div>\n    </div>\n  `;\n\n  container.innerHTML = styles + html;\n\n  const tarotCard = container.querySelector('.tarot-card');\n  if (tarotCard) {\n    tarotCard.addEventListener('mousemove', function(e) {\n      const rect = tarotCard.getBoundingClientRect();\n      const x = e.clientX - rect.left;\n      const y = e.clientY - rect.top;\n      const centerX = rect.width / 2;\n      const centerY = rect.height / 2;\n      let rotateX = (y - centerY) / 10;\n      let rotateY = (centerX - x) / 10;\n      const maxAngle = 10;\n      rotateX = Math.max(-maxAngle, Math.min(maxAngle, rotateX));\n      rotateY = Math.max(-maxAngle, Math.min(maxAngle, rotateY));\n      tarotCard.style.transform = 'rotateY(' + rotateY + 'deg) rotateX(' + rotateX + 'deg) scale(1.05)';\n      tarotCard.style.filter = 'drop-shadow(0 30px 60px rgba(0,0,0,0.6))';\n    });\n    tarotCard.addEventListener('mouseleave', function() {\n      tarotCard.style.transform = 'rotateY(0deg) rotateX(0deg) scale(1)';\n      tarotCard.style.filter = 'drop-shadow(0 20px 40px rgba(0,0,0,0.5))';\n    });\n  }\n  const langBtn = container.querySelector('.lang-switch');\n  const messageDiv = container.querySelector('.card-message-text');\n  let currentLang = 'eng';\n  if (langBtn && messageDiv) {\n    langBtn.addEventListener('click', function() {\n      if (currentLang === 'eng') {\n        messageDiv.textContent = cardData.message_cn;\n        langBtn.textContent = '🇺🇸';\n        currentLang = 'cn';\n      } else {\n        messageDiv.textContent = cardData.message_eng;\n        langBtn.textContent = '🇨🇳';\n        currentLang = 'eng';\n      }\n    });\n  }\n}\n\nif (typeof document !== 'undefined') {\n  document.addEventListener('DOMContentLoaded', function() {\n    createTarotCard('tarot-container');\n  });\n}");

    Ok(js)
  }
} 