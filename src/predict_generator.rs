use std::string::String;
use sha2::{Sha256, Digest};

pub fn generate_prediction(index: u128) -> (String, String) {
    let future = [
        "Today", "Tomorrow", "This Morning", "This Afternoon", "This Evening", "Tonight", 
        "Next Monday", "Next Tuesday", "Next Wednesday", "Next Thursday", "Next Friday", 
        "Next Saturday", "Next Sunday", "This Weekend", "Next Week", "In Two Days", 
        "By Noon", "At Night", "Before Bed", "After Breakfast", "During Lunch", 
        "After Work", "Before Dinner", "At Sunrise", "At Sunset", "In the Morning", 
        "In the Evening", "This Night", "Next Morning", "Next Evening", "On Monday", 
        "On Tuesday", "On Wednesday", "On Thursday", "On Friday", "On Saturday", 
        "On Sunday", "This Week", "Next Weekend", "In Three Days", "By Evening", 
        "After Lunch", "Before Noon", "At Midnight", "At Dawn", "At Dusk", 
        "Before Sunrise", "After Sunset", "During Breakfast", "During Dinner", 
        "After Midnight", "Before Midnight", "In Four Days", "This Month", 
        "Next Month", "By Tomorrow", "By Tonight", "This Day", "Next Night"
    ];
    let future_cn = [
        "今天", "明天", "今天早上", "今天下午", "今天晚上", "今晚", 
        "下周一", "下周二", "下周三", "下周四", "下周五", 
        "下周六", "下周日", "本周末", "下周", "两天后", 
        "中午前", "夜里", "睡前", "早餐后", "午餐时", 
        "下班后", "晚餐前", "日出时", "日落时", "早上", 
        "晚上", "今夜", "明早", "明晚", "周一", 
        "周二", "周三", "周四", "周五", "周六", 
        "周日", "本周", "下个周末", "三天后", "傍晚前", 
        "午饭后", "中午前", "午夜", "黎明", "黄昏", 
        "日出前", "日落后", "早餐时", "晚餐时", 
        "午夜后", "午夜前", "四天后", "本月", 
        "下月", "明天前", "今晚前", "今天", "明夜"
    ];
    let subject = [
        "You", "Your Friend", "Your Boss", "Your Colleague", "Your Neighbor", "Your Partner", 
        "Your Family", "Your Team", "Your Rival", "A Stranger", "Your Mentor", 
        "Your Teacher", "Your Coworker", "Your Sibling", "Your Parent", "Your Child", 
        "Your Best Friend", "Your Enemy", "Your Classmate", "Your Manager", 
        "Your Client", "Your Roommate", "Your Spouse", "Your Crush", "Your Ex", 
        "Your Doctor", "Your Accountant", "Your Coach", 
        "Your Barber", "Your Driver", "Your Waiter", "Your Barista", "Your Therapist", 
        "Your Neighbor’s Dog", "Your Cat", "Your Dog", "Your Pet", "Your Landlord", 
        "Your Tenant", "Your Mailman", "Your Grocer", "Your Pharmacist", "Your Banker", 
        "Your Mechanic", "Your Plumber", "Your Electrician", "Your Dentist", 
        "Your Hairdresser", "Your Tailor", "Your Cleaner", "Your Gardener", 
        "Your Babysitter", "Your Nanny", "Your Tutor", "Your Advisor", 
        "Your Accountant", "Your Chef", "Your Assistant"
    ];
    let subject_cn = [
        "你", "你的朋友", "你的老板", "你的同事", "你的邻居", "你的伴侣", 
        "你的家人", "你的团队", "你的对手", "陌生人", "你的导师", 
        "你的老师", "你的同事", "你的兄弟姐妹", "你的父母", "你的孩子", 
        "你最好的朋友", "你的敌人", "你的同学", "你的经理", 
        "你的客户", "你的室友", "你的配偶", "你的暗恋对象", "你的前任", 
        "你的医生", "你的会计", "你的教练", 
        "你的理发师", "你的司机", "你的服务员", "你的咖啡师", "你的治疗师", 
        "你邻居的狗", "你的猫", "你的狗", "你的宠物", "你的房东", 
        "你的租客", "你的邮递员", "你的杂货商", "你的药剂师", "你的银行家", 
        "你的技工", "你的水管工", "你的电工", "你的牙医", 
        "你的美发师", "你的裁缝", "你的清洁工", "你的园丁", 
        "你的保姆", "你的保育员", "你的家教", "你的顾问", 
        "你的会计", "你的厨师", "你的助理"
    ];
    let verb = [
        "Will Triumph", "Will Flop", "Will Dominate", "Will Stumble", "Will Discover", "Will Reveal", 
        "Will Cackle", "Will Weep", "Will Hustle", "Will Chill", "Will Wander", "Will Linger", 
        "Will Clash", "Will Spark", "Will Soothe", "Will Snub", "Will Rally", "Will Grumble", 
        "Will Scheme", "Will Ditch", "Will Invent", "Will Wreck", "Will Mend", "Will Shatter", 
        "Will Snag", "Will Trade", "Will Holler", "Will Ping", "Will Ghost", "Will Recall", 
        "Will Blank", "Will Push", "Will Bail", "Will Launch", "Will Wrap", "Will Stall", 
        "Will Sprint", "Will Unwind", "Will Freak", "Will Sizzle", "Will Feast", "Will Crash", 
        "Will Groove", "Will Croon", "Will Brawl", "Will Grovel", "Will Absolve", "Will Cram", 
        "Will Rage", "Will Stash", "Will Splurge", "Will Rake", "Will Misplace", "Will Uncover", 
        "Will Charm", "Will Fumble", "Will Shine", "Will Flirt", "Will Roast", "Will Swagger"
    ];
    let verb_cn = [
        "将成功", "会失败", "将主宰", "会绊倒", "会发现", "会揭示", 
        "会大笑", "会哭泣", "会奋斗", "会放松", "会徘徊", "会逗留", 
        "会冲突", "会激发", "会安慰", "会冷落", "会集结", "会抱怨", 
        "会策划", "会抛弃", "会发明", "会破坏", "会修复", "会粉碎", 
        "会抓住", "会交易", "会呼喊", "会联系", "会消失", "会回忆", 
        "会空白", "会推动", "会退出", "会启动", "会结束", "会拖延", 
        "会冲刺", "会放松", "会惊慌", "会滋滋作响", "会盛宴", "会崩溃", 
        "会律动", "会歌唱", "会争吵", "会卑躬屈膝", "会赦免", "会死记", 
        "会愤怒", "会藏匿", "会挥霍", "会耙拉", "会遗失", "会发现", 
        "会吸引", "会失误", "会闪耀", "会调情", "会烤制", "会炫耀"
    ];

    let mut hasher = Sha256::new();
    hasher.update(index.to_le_bytes());
    let hash = hasher.finalize();

    let f_idx = (u64::from_le_bytes(hash[0..8].try_into().unwrap()) % future.len() as u64) as usize;
    let s_idx = (u64::from_le_bytes(hash[8..16].try_into().unwrap()) % subject.len() as u64) as usize;
    let v_idx = (u64::from_le_bytes(hash[16..24].try_into().unwrap()) % verb.len() as u64) as usize;

    let f = &future[f_idx];
    let s = &subject[s_idx];
    let v = &verb[v_idx];
    let f_cn = &future_cn[f_idx];
    let s_cn = &subject_cn[s_idx];
    let v_cn = &verb_cn[v_idx];
    (
        format!("{} • {} • {}", f, s, v),
        format!("{} • {} • {}", f_cn, s_cn, v_cn)
    )
}