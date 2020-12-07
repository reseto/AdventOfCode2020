use std::time::{Instant};

fn part_a(input: &String) {
    let bags = parsed(input);
    let mut searches = Vec::new();
    searches.push("shinygold");
    let mut past_searches: Vec<&str> = Vec::new();
    let mut found = Vec::new();
    loop {
        if searches.is_empty() { break; }
        let search = searches.pop().unwrap();

        for bag in &bags {
            if bag.co.contains(search) && ! found.contains(bag) && ! past_searches.contains(&search) {
                found.push(*bag);
                // println!("pushed name: {}", bag.name);
                searches.push(&bag.name);
            }
        }
        past_searches.push(search);
    }
    println!("answer a: {}", found.len());
}

fn part_b(input: &String) {
    let bags = parsed(input);
    println!("answer b: {}", go_deep(&bags, "shinygold"));
}

fn go_deep(bags: &Vec<Bag>, name: &str) -> i32 {
    let mut itr = bags.iter();
    let bag = itr.find(|b| b.name == name).unwrap();
    // name = shinygold
    // shinygold#5palebrown,2lightred,3drablime
    // co = 5palebrown,2lightred,3drablime
    // I'm a lucky bastard that it's always 1 digit in front of the name
    let mut sum = 0;
    for pair in bag.co.split(',') {
        if pair != "noother" {
            let count:i32 = pair[0..1].parse().unwrap();
            let name2:&str = &pair[1..];
            sum += count + count * go_deep(bags, name2) // count + 0 for noother;
            // println!("going deep on name {} with name2 {} count {} sum {}", name, name2, count, sum);
        } 
    }
    sum
}

fn main() {
    // let input = _example();
    let input = converted();

    let start_a = Instant::now();
    part_a(&input);
    let start_b = Instant::now();
    part_b(&input);
    println!(" A took {:?}", start_a.elapsed());
    println!(" B took {:?}", start_b.elapsed());
}

fn parsed(input: & String) -> Vec<Bag> {
    let mut vec = Vec::new();
    for line in input.lines() {
        let mut s1 = line.split('#');
        let name = s1.next().unwrap();
        let contents = s1.next().unwrap();
        let bag = Bag {
            name: name,
            co: contents
        };
        vec.push(bag);
    }
    vec
}

#[derive(Debug, Copy, Clone, Eq, Hash)]
struct Bag<'a> {
    name: &'a str,
    co: &'a str
}

impl PartialEq for Bag<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

fn _example() -> String {
    let input = r########"lightred#1brightwhite,2mutedyellow
darkorange#3brightwhite,4mutedyellow
brightwhite#1shinygold
mutedyellow#2shinygold,9fadedblue
shinygold#1darkolive,2vibrantplum
darkolive#3fadedblue,4dottedblack
vibrantplum#5fadedblue,6dottedblack
fadedblue#noother
dottedblack#noother
"########;
    input.to_string()
}

fn _example2() -> String {
    let input = r########"shinygold#2darkred
darkred#2darkorange
darkorange#2darkyellow
darkyellow#2darkgreen
darkgreen#2darkblue
darkblue#2darkviolet
darkviolet#noother
"########;
    input.to_string()
}

fn converted() -> String {
    let input = r########"poshblue#5plaidchartreuse,3plaidlime
clearteal#2dottedsalmon,2wavyred
fadedblue#1dottedchartreuse,3dimbronze
plaidblack#5mutedbeige,2palegold,3wavylavender,5dullyellow
brightcyan#2vibrantteal
clearmagenta#2dimchartreuse
mutedcrimson#1clearviolet,5darkcoral,1palesalmon,3lightred
dottedgreen#3mutedplum
palecrimson#3palemaroon,2mirroredtan
shinyblack#1wavytomato
stripedfuchsia#1lightchartreuse,2stripedturquoise,1dimblue,1lightteal
drabgreen#5dottedtan,3dottedturquoise,2clearcoral,5vibrantbronze
brightfuchsia#4darkturquoise
mirroredsilver#1dottedlavender,5lightblack,2clearbeige
darktomato#3drabcyan,1wavycrimson
brightcrimson#3mirroredolive,2vibranttan,3shinycrimson
lightmaroon#4brightlime,3wavypurple
fadedlavender#4stripedcrimson
lightwhite#3shinygray,1brightgold,1lightyellow
poshsilver#1brightlavender,3brightchartreuse
fadedsalmon#3stripedgreen,3dulllavender,1stripedmaroon
drabchartreuse#2lightplum,2brightplum,2palemaroon,1wavyred
brightgreen#4darkmagenta
stripedbeige#1fadedred,2mutedgold,2shinytomato
mutedbeige#1plaidtan
wavygreen#1clearblue,1stripedgold
shinymaroon#2mutedorange,4dullbeige,2shinycrimson
fadedyellow#1dullgold,3brightfuchsia,1darkblack
dottedcrimson#5poshblue,3drabsilver,5mutedturquoise,2brightindigo
lightolive#4drabtomato,3drabturquoise,5plaidfuchsia
darkpurple#4dottedblue,1drabcrimson,3drabred
brightlime#3mirroredorange,1wavyfuchsia,3mutedplum
mutedyellow#1dottedwhite,2plaidgreen
cleargold#5darkgray,5dottedtomato,1mirroredaqua
dullolive#1vibrantgreen,3wavyfuchsia,4dullorange
dullteal#2vibrantcrimson,5shinyyellow
poshgold#2mutedyellow,1poshindigo,4brightsalmon
clearcrimson#5mirroredsilver,5mutedred
fadedindigo#3lightturquoise,5fadedred,2wavycrimson
brightbronze#1brightred,5brightlime,1darkgreen
vibrantcrimson#1wavyteal,2vibrantbrown
wavyaqua#4mirroredwhite
mirroredwhite#4shinyteal,5lightturquoise,2fadedcoral
brightgray#1mutedgold,4vibrantteal
lighttomato#5plaidbronze,2lightviolet
wavytan#5darkcoral,3clearbeige,2fadedorange
mutedindigo#2wavyfuchsia
fadedtan#5fadedviolet,4paleturquoise,4vibrantbronze,5dottedlime
dottedmagenta#5brighttomato
mirroredcyan#4plaidcrimson
darkbeige#1clearteal
poshorange#1poshindigo,1shinyaqua,4fadedgray,3dottedlavender
palechartreuse#4clearviolet,4clearorange
dottedbronze#3palebrown,1plaidturquoise,2dimfuchsia
dimcyan#2brightsilver
lightcrimson#4dimcrimson
vibrantbrown#5darkolive,4palesalmon
dimtan#5plaidbronze,4palemagenta,4dullbronze
shinytomato#noother
stripedred#4vibrantolive,4poshblack,4brightgold,2vibrantcoral
brightmagenta#1dulltomato,5brightlime,3mirroredwhite
poshmaroon#5plaidbronze,3brightsilver,3plaidchartreuse
vibranttan#3plaidchartreuse,4stripedsilver
stripedlime#2dullbeige,4plaidorange
wavyturquoise#1stripedteal,4dimorange,3fadedsilver,1shinypurple
clearcoral#5paleyellow,1brightsilver,1cleartan
lightviolet#1poshbronze
wavylime#4shinycoral,2fadedbronze,2drabteal
dottedwhite#5wavyfuchsia
dimblue#1drabblack
drabsalmon#5dullgreen,2mirroredolive
clearbrown#5brightmaroon,4drabteal,4clearbronze
lightturquoise#3shinyaqua,1mutedred,2wavyteal
plaidcrimson#3stripedtomato,3plaidchartreuse,4lightgreen
mutedpurple#5drabplum,5drabsalmon,1vibranttan
palelime#3fadedindigo,5darkindigo,4dullblue,1fadedlavender
stripedorange#1vibrantred,1mirroredolive,5shinyblue,3brighttomato
poshpurple#3palebrown,5plaidturquoise,4dullaqua,5fadedlavender
vibrantlavender#2vibrantmaroon,4mutedolive,3clearcoral
wavypurple#3mutedgold,3poshlime
dullcrimson#4drabviolet,5wavyteal,4paleindigo,1palewhite
poshplum#2mutedblack,2clearmaroon,3dottedred
dulltan#3brightlavender,1dullgreen,2wavyteal,5plaidbronze
paletan#4palefuchsia,3drabpurple
palered#4vibrantbronze
stripedturquoise#5fadedbrown
clearcyan#4brightbronze,5mirroredpurple
dottedmaroon#4plaidsilver,4plaidyellow,1darkchartreuse,1stripedbronze
brightlavender#4mirroredblack,5shinybronze,2wavycrimson
clearfuchsia#2drabblack,3fadedyellow
dottedlime#5stripedpurple
mirroredbeige#3dimfuchsia
poshlime#2shinytomato,1darkbrown,1brighttomato,5palesalmon
brightmaroon#5clearviolet,1clearorange,5mirroredolive
lightlavender#5mutedpurple,5shinyaqua,2plaidcrimson,1mutedwhite
wavycoral#5brightmaroon
stripedgreen#4palesalmon,5mirroredolive,3darkcoral,2plaidwhite
vibrantcyan#4dottedgreen,4brighttomato,4lightfuchsia,2fadedbronze
shinymagenta#3mirroredlavender,4brighttomato,2darkbrown,5vibrantblack
shinyolive#1shinymagenta,2dimolive
poshchartreuse#3mutedblack,2poshyellow
dullred#5mirroredbeige,2dottedbronze,2dimbrown,5dottedtan
dullcyan#1fadedbronze,4shinyplum,2dimolive,3fadedolive
paleplum#1dottedblack
mirroredblack#5shinybronze,2drablime,1fadedred,2mirroredorange
mirroredchartreuse#4dottedplum
fadedbronze#1palebronze,5brightchartreuse,3plaidgreen,4drablime
mutedolive#2mirroredtan
clearred#4dullfuchsia
plaidmagenta#4stripedgray,5dimcyan
darksalmon#3mirroredgray,2mutedblack
plaidcoral#3mirroredindigo,5dottedfuchsia,5paleblack,2fadedcrimson
dullpurple#1drabgold,3plaidtan,5poshindigo
stripedwhite#5plaidpurple,5darkcoral,4dullbeige,5poshbronze
brightsilver#3palebrown,4lightblack
darkbronze#4wavycrimson,4wavytan,3vibrantteal
wavyblue#2clearcoral
stripedteal#2stripedbronze,4stripedcrimson
dullbronze#4stripedbronze,4dimlavender
poshsalmon#4fadedbronze,3wavycrimson,5plaidtan
draborange#1wavyyellow,4dullbronze,5dullchartreuse
drabcyan#1darklime,4fadedtomato,3palebrown,2poshbronze
fadedmagenta#1shinycoral,2fadedsilver,2brightsilver
darkgreen#5paleaqua,5mutedolive,4mirroredsalmon,1lightmagenta
plaidblue#5drablavender,2dullaqua
wavycyan#4poshgold,4lightteal
dullblack#5drabsilver,5lightcyan,4dimbrown,3wavyteal
fadedblack#2plaidmagenta,4lightchartreuse,5wavygold
dimsilver#4wavycrimson,3clearbronze,3mirroredwhite
lightcoral#2plaidgray,3vibrantviolet,3dimfuchsia,1drabmagenta
palebeige#3dimteal,1lightbeige
brightindigo#4poshfuchsia,1poshred
dottedfuchsia#4mutedblack,3poshturquoise,1plaidtan,2lightsilver
dottedtan#3drabtomato
lightsilver#4palebrown,1mirroredorange
mutedsilver#2wavyfuchsia,2shinybronze,5darkolive
dimbeige#1lightwhite,2mirroredgold,2dullgold
fadedcyan#5cleargray,5poshorange,2poshviolet,4clearchartreuse
mutedsalmon#3drabplum
darkturquoise#1brighttomato,2plaidgreen,3mutedturquoise
shinygreen#3dottedsalmon,4fadedolive,1dimmaroon,4mutedlavender
shinybeige#1drabsalmon,4dullgreen,5clearred,4mutedbrown
palebrown#5stripedgreen
poshtan#3brightyellow
shinygray#4fadedorange,5wavyaqua
lightaqua#4dottedwhite,2dottedblack
dimyellow#5wavylime,2fadedmaroon,5wavyteal
stripedbronze#4shinyorange,1mirroredbeige,1wavymaroon
stripedtan#4lightplum
dimlime#1wavycoral,4lightplum,5poshtan,2darkblack
dottedaqua#1shinyteal
shinybrown#3plaidyellow,3vibrantindigo,1mutedtan,2shinytomato
drabyellow#1drablime,4dullbeige,3plaidgray
stripedtomato#5shinygold
brightolive#5dottedred,2wavygreen,4drabmagenta
mirroredbronze#4drabcyan,2lightchartreuse,1shinyyellow
wavygold#5paleyellow,1brightaqua,5stripedcrimson
wavysilver#4plaidgreen
clearyellow#2plaidyellow,4shinygold,2drabmagenta,2brightred
darkgold#5mirroredcyan,4stripedbeige,4mirroredolive,5mutedgold
vibrantbronze#3clearorange
brightpurple#2brightlime,1mirroredturquoise
dullcoral#3stripedtomato
brightcoral#5darklime,4poshviolet,3drabchartreuse,3dullblue
darkmagenta#4palebrown
lightteal#3stripedpurple
stripedlavender#3plaidtomato,1darktomato,3mirroredblack,4vibrantbronze
lightred#5darkbrown
dullviolet#3brightindigo,4plaidtan,4fadedlime,3mutedaqua
wavycrimson#4mirroredolive,1brighttomato,5stripedsilver,1darksalmon
fadedtomato#3brightsilver
shinyfuchsia#2drabcrimson
fadedcrimson#3brightlavender,4brightsalmon,4paleaqua
dottedblue#3shinychartreuse
dullturquoise#2shinybronze,2stripedgreen,5poshbronze
vibrantsalmon#2dimcyan,3dimbronze,5brightlavender
stripedcrimson#4dullbeige,2poshorange
paleyellow#2mirroredblack,1mutedsilver,2paletomato,2mutedplum
mirroredindigo#4drabolive,4dullbeige
poshlavender#4vibrantolive,2clearred
palelavender#3lightred,5dimchartreuse,5dullgreen
mutedlavender#2brightmaroon
mirroredblue#4poshbronze
darkviolet#4brightred,4mirroredyellow,5cleargray,3darkbrown
dimtomato#5mutedcrimson,1dimblue,1brightcrimson,5shinylime
fadedsilver#1mirroredblack,2mutedindigo,4stripedpurple,5vibrantblack
dullgold#5stripedwhite,2brightbeige
dullchartreuse#1shinymagenta,5brightlavender,5fadedmaroon,5shinyorange
plaidtan#1shinymaroon,1palewhite,3darklime
poshtomato#4fadedcoral,2dullchartreuse,4shinyteal,3lightbeige
vibrantblue#4darkteal,4poshteal
plaidbronze#5poshlime,1darkcoral,4fadedred
lightpurple#5dimcrimson
vibrantlime#1paleturquoise,5mutedturquoise
mutedorange#3darkfuchsia
dimwhite#1mutedteal,3darkwhite,4lightcyan
vibrantpurple#4vibrantred,4poshgold,1vibrantcrimson,2mutedplum
plaidplum#1lightplum,4vibrantorange
clearplum#3clearyellow,3fadedsilver,1vibrantred,3wavylavender
stripedgray#4wavychartreuse,1clearcrimson,3stripedbeige
fadedgold#4vibrantyellow,5dottedindigo
fadedmaroon#3drablime,5plaidgray,2mutedsilver,2vibrantcrimson
brightsalmon#1dimteal,2plaidgreen
vibrantorange#4mutedolive
palecoral#4mutedaqua,3brightindigo,4dimcyan
drabgray#4poshblue,5shinycrimson
mutedcoral#5plaidgray,4wavyteal,3mutedturquoise
clearmaroon#3vibrantviolet
drabviolet#3poshorange,1mirroredorange
dottedplum#4plaidyellow,4poshlime
shinyviolet#3fadedindigo,3mutedtan
plaidgray#2lightbeige,3shinybronze
dottedcyan#1wavymaroon,1dullcoral
wavybeige#2brightmaroon,4poshteal,2shinygold,3palepurple
mutedgold#5stripedgold,3poshbronze
mirroredcrimson#4dullteal,4plaidgray
wavyolive#4darktan
dimorange#4vibrantmagenta,3plaidteal
mirroredorange#4shinytomato,4wavyfuchsia,5fadedred
drabcrimson#2vibrantcrimson,2cleargray
stripedaqua#2drabsilver,5vibrantfuchsia
vibrantindigo#4mutedmaroon
poshindigo#5clearviolet,5brightlime
drabplum#3paleyellow,4fadedorange,5mutedbrown
mirroredfuchsia#3fadedolive,4dullblack,1plaidcyan,2dottedfuchsia
drabaqua#1wavyblue,2plaidorange,4fadedlavender
dimcrimson#2lightbrown
fadedwhite#3wavypurple,1clearyellow
dimplum#1dottedplum
dimindigo#5wavypurple,3plaidgreen
dimmaroon#1dimcyan,1vibrantolive,4mutedtomato,1vibrantbrown
shinysalmon#1mutedyellow,2clearsilver
poshviolet#1poshfuchsia,4brightred,2plaidmaroon,2brightbeige
lightgold#2plaidolive
dottedteal#3drabteal,4brightgold,2clearlavender
darkwhite#2wavyred,5poshorange
mutedblack#5darkcoral
dimred#4mutedblue
plaidfuchsia#2stripedwhite
clearblue#1dimyellow,3fadedindigo,5cleartan,2clearcrimson
clearsalmon#3darkindigo
fadedred#noother
stripedmaroon#2mutedblue,3darkolive,5stripedwhite
vibrantgold#5mutedcrimson,5clearmagenta,5fadedindigo,5vibrantcoral
wavyred#2stripedbeige
paleindigo#3stripedbronze,3paleblue
mutedbronze#4brightcrimson,4mutedorange,4wavyred
drabred#4vibrantbrown,3fadedred,4mirroredolive
darksilver#4dullchartreuse
dimolive#3plaidwhite
plaidolive#4mirroredgray
vibrantsilver#4wavymaroon,1brightgold
dimteal#5mirroredred
vibrantfuchsia#3plaidlime,3shinyorange
vibrantaqua#5wavywhite,2mutedlavender,1poshgold
stripedolive#5brightbronze,4shinychartreuse
clearpurple#3darkturquoise,1drabyellow,2lightgray
poshgreen#4dottedtomato
fadedfuchsia#4plaidtan,2wavyteal,4shinygold
shinychartreuse#3darkolive,5dimlavender,5dottedwhite
plaidteal#3vibrantcrimson,4clearorange,5plaidturquoise
drabbrown#1stripedsilver,1wavylime,4dottedwhite
plaidindigo#3poshgray,3dottedturquoise,5dottedtomato
vibrantyellow#1fadedbeige,2vibrantred
poshwhite#2poshyellow,2drabturquoise
poshyellow#5clearplum
dottedorange#1brightcrimson,5mirroredturquoise,2lightplum,4dimturquoise
dullgreen#2vibrantteal,4mutedturquoise,2plaidwhite
stripedchartreuse#3lightviolet,4palebrown,2lightolive,3mutedred
darkcrimson#3dimmagenta,2paleolive,2vibrantsilver,4poshgold
mutedviolet#3dimwhite,4mutedorange
mutedblue#2shinyorange,4clearviolet,1shinybronze
cleartan#4mutedplum,4darkfuchsia,4mutedindigo
paleturquoise#5shinybronze
stripedindigo#5fadedfuchsia,5dottedbronze,2stripedgreen,2fadedcoral
shinylavender#3shinygold,2shinymagenta
clearlime#2dottedsalmon,2dulllime,2darkgreen,1dimblack
dullfuchsia#4darkfuchsia
fadedbeige#4mirroredblue,1wavycrimson,5vibrantred,1drabteal
posholive#1mutedblue,4mutedyellow,3clearbronze,4dimfuchsia
palegray#5shinyplum,2stripedgreen,3darkcoral,5stripedcrimson
clearaqua#2poshgray,3shinyred,4poshturquoise
dimturquoise#1dullyellow
plaidcyan#2mirroredcyan
mutedgray#3fadedolive,1fadedbronze
dottedviolet#1clearplum,1darkaqua
drabturquoise#5brightgray,3shinymaroon,3dullaqua
dullwhite#2shinymaroon,2dimolive,5dullturquoise
fadedturquoise#1plaidlime
poshbronze#noother
dottedbrown#4darkblack,5plaidgray
dottedgold#5stripedplum,4dimsalmon,4lightblack,3stripedindigo
darkplum#5dimturquoise
mirroredviolet#5clearplum,3stripedorange,4dullgreen,3lightbeige
cleargray#3stripedsilver,1mutedturquoise
mirroredmaroon#4darkolive
poshbeige#2mutedbeige,4drabtomato
mutedmaroon#1clearviolet
mirroredcoral#2darkturquoise,4brightfuchsia,5vibrantblack,3stripedcyan
drabbronze#5dimtan,2lightviolet,3brighttan,3brightaqua
dullyellow#3paleyellow,5brightmaroon,3stripedtomato,4mutedbrown
brightbrown#5darkbronze
mutedlime#5clearviolet
dimbrown#5plaidolive
vibrantchartreuse#2poshred,1fadedlime,4wavybeige
fadedorange#4shinymagenta,5brightsilver,3dullcoral
dimlavender#3darkbrown,3fadedindigo,1vibranttan
darkolive#3palesalmon,5poshbronze
brightred#2cleargray
mutedfuchsia#2palered,4dottedblue,1dimolive,4palesalmon
mirroredred#4vibrantcrimson,4wavyteal,2fadedgray,2darkbrown
paleaqua#2dimlavender,5dimteal,2fadedcoral
lightlime#2plaidbeige,2wavylime,2mirroredaqua,4mutedturquoise
shinyturquoise#2dottedlavender,3vibranttan,5lightblack
plaidsalmon#3dottedpurple,2palelavender,3wavygold,1clearcrimson
lightblack#1mutedsilver,3palebronze,3palesalmon
stripedbrown#3brightmagenta,5fadedorange,5wavybeige,2plaidorange
palegreen#4mirroredbeige,5fadedindigo,5paletomato
clearolive#4wavypurple,5paleolive
plaidtomato#1dottedgreen
paletomato#5lightred,1brightmaroon,2stripedgreen,3plaidturquoise
wavylavender#3darkbrown
dottedturquoise#2vibrantfuchsia,3poshgold,1mirroredolive,2brightsalmon
mirroredmagenta#3mutedblack,1dimchartreuse,1dimfuchsia,1lightturquoise
paleblack#1paletomato
shinycyan#3vibrantcoral,3shinybronze,5mutedgreen
fadedchartreuse#1plaidolive,1dimplum
mirroredteal#2darkmaroon,1brightbeige,1brighttan,2lightviolet
stripedgold#5clearviolet,3mutedturquoise
lightsalmon#2fadedbrown,5palelavender,2vibrantlime
darkbrown#5mutedturquoise,3plaidwhite
shinytan#1lightcyan,2wavyred,4dullpurple
dulltomato#4dottedlavender,5mutedgreen,1dullgreen
plaidmaroon#4brightplum,1mutedmaroon,3stripedpurple,3poshgray
stripedblue#4palemagenta,2plaidlime
dottedindigo#1brightfuchsia
drabteal#2darkbrown,3mutedblue
clearlavender#3poshgray,1plaidpurple,2mirroredviolet,4dulllime
palefuchsia#4darksilver,5clearyellow,4lightblack
fadedgreen#3dullcrimson,4dottedindigo,3dullmaroon,4mutedblue
clearwhite#4drabcrimson,3shinycrimson,1vibrantindigo,1lightturquoise
brightaqua#4dottedbronze,3poshgold,4palebeige,5clearviolet
palegold#5wavyred
mirroredyellow#1brightwhite,4fadedgray,3darkturquoise,2cleargray
drabblack#3vibrantgreen
plaidpurple#1darksalmon,4lightred
drabgold#3darkfuchsia
drabfuchsia#1shinybronze,3mirroredlime
clearblack#1fadedcoral,4dulltan
fadedpurple#2clearmagenta
brightyellow#2shinyyellow,4stripedplum,1mirroredsilver,1plaidaqua
palemaroon#4clearbronze
dullplum#5palebeige,2paleolive,5darkturquoise,1dullgold
mirroredturquoise#1darklime
brightorange#2palegreen,4plaidwhite,3mutedgreen
plaidred#1dottedturquoise,3darkwhite
palebronze#3poshlime,3clearbeige,3stripedgold
vibrantcoral#1stripedgreen,4mutedcrimson,2vibrantbrown
plaidorange#5fadedbronze
poshbrown#4mutedaqua,3dulllavender,2mirroredgold,1lightbrown
lightblue#4wavyteal,1poshturquoise
fadedolive#3drabsilver,1drabgreen
poshcoral#2shinycrimson
mirroredlavender#1wavyteal,5dulllime,5plaidturquoise,5palebrown
darkblue#4mirroredsilver,5vibrantblack,2paleorange
dottedpurple#3poshfuchsia,4wavybeige
lightyellow#3palemagenta
poshaqua#2mutedwhite,2mutedbeige,4stripedorange
clearturquoise#5drabplum
mirroredplum#1fadedgray,4dimbrown,2brightviolet,5brightbeige
brighttan#1poshsalmon,3dottedred,1vibrantred
brightplum#3palechartreuse,4plaidbronze,4fadedgray,1mutedtomato
lightindigo#5dullbeige,1stripedchartreuse,5vibrantcoral,2paleturquoise
dullaqua#4drabindigo,5vibrantblack
dullbrown#1dottedblack,1clearolive,3dullaqua,1drabmagenta
plaidwhite#noother
mirroredolive#noother
mirroredgreen#3clearteal,4clearmagenta,1palecrimson
mutedtomato#1lightmagenta,1palebrown
mutedwhite#5stripedteal,2drabbrown
darkindigo#1brightsilver,5brightgold,2brightmaroon
darkfuchsia#2plaidwhite,3vibrantbrown,2plaidgray
stripedcyan#2dimchartreuse,3mirroredsilver,3dimcyan
brightviolet#2dimteal,5vibrantpurple,1brightblack
plaidaqua#3lightchartreuse
fadedaqua#4wavycrimson,1palebrown,2poshsilver
mutedchartreuse#3shinypurple,1drabtomato
plaidchartreuse#1mutedindigo,5brightmaroon,4fadedmaroon
wavymagenta#4lightwhite
dimviolet#5drabred,5mutedplum,4stripedcrimson
darkmaroon#5lightteal,4drabtan,5lightaqua,3dimmaroon
fadedcoral#2mutedcrimson
mirroredlime#3poshorange,5plaidyellow,1vibrantolive,1brightlime
shinyyellow#2vibrantgold,2mutedorange,4wavysalmon
brightgold#1cleargray,4poshteal,2vibrantcoral,3dottedsalmon
plaidlime#4plaidgreen,1brightmaroon
mirroredaqua#4stripedgray,3vibrantolive,2dottedtan,5shinylime
lightgreen#3dimlavender
mutedturquoise#noother
mutedaqua#2fadedpurple,2clearchartreuse,5poshteal,4plaidorange
drabtomato#5mirroredolive,1mutedturquoise,1stripedsilver
drabtan#5shinypurple
vibrantturquoise#2poshbeige
stripedcoral#3stripedplum,2mirroredchartreuse,4palelavender
mirroredbrown#1plaidsalmon,3mirroredred
clearbronze#4dullturquoise,1wavyteal,1dullgreen
stripedsilver#3mutedcrimson,5clearorange
dimgold#2stripedorange,1brightviolet
shinyblue#4palemagenta,4drablime
clearorange#noother
wavyindigo#4fadedolive,3dottedcrimson,2clearblack
drabbeige#3mirroredsalmon,4brightlavender
dottedyellow#4mirroredbrown,4fadedred,4dottedgray,3plaidgray
stripedpurple#4vibrantteal,3darkfuchsia,3brightchartreuse,3dullgreen
plaidgold#4shinyaqua,1wavytomato,1fadedyellow
dimbronze#5darkcoral,5shinybeige,5poshindigo
drablime#4plaidgreen,1darksalmon
vibrantmagenta#2shinymagenta,5shinyyellow,1vibrantindigo
shinypurple#1wavyteal
palepurple#1mirroredsilver,2clearbeige
palemagenta#2mirroredsilver,1shinygold,3dottedtomato,4dottedlavender
plaidturquoise#5fadedbronze,1stripedgold,3darksalmon
dottedchartreuse#1mirroredorange,4paleturquoise,3mutedbeige
stripedblack#5dullbeige,2clearred
wavyviolet#3brightplum,5dimmaroon
darkaqua#5shinycyan,4vibrantbronze
shinyplum#3dottedlavender,1clearred,3paleyellow,2mutedcrimson
poshfuchsia#1stripedtomato,1dulltan,4darkturquoise,4mutedmaroon
darktan#1drabsalmon,1mutedcyan
lightfuchsia#4dimred,1lightsilver
paleblue#3lightbeige,1dottedlavender,4paletomato,3stripedwhite
plaidbrown#4brightviolet,1lightgreen,4stripedlime
shinywhite#4clearsilver
darkteal#3mutedgold
fadedlime#5mirroredplum,2fadedred,4dullorange
mirroredsalmon#3mutedturquoise,1darksalmon
brightchartreuse#2mutedred,5mutedmaroon,1vibrantbrown,4darkcoral
dottedblack#1stripedwhite,5dulltan
dullmaroon#1fadedmaroon,1poshsilver,2dottedturquoise
wavybronze#2dimindigo,1mirroredbeige
wavychartreuse#4fadedsilver,4mirroredolive
vibrantred#4shinytomato,5mirroredgray,2plaidgray
dulllavender#2mutedlavender
shinyred#2fadedmagenta
mutedcyan#1plaidbronze
drabwhite#5lightturquoise,1stripedcoral,3wavyteal
brightblack#1wavyfuchsia,3drabyellow
shinyaqua#4mirroredorange,3wavyfuchsia
paleviolet#3dimfuchsia,4vibrantviolet
wavybrown#4darkviolet
wavyplum#4darkblack,2lightorange,1poshpurple
shinyindigo#3dullyellow,1stripedlime,3stripedpurple
darkchartreuse#3darkturquoise,5darkgold,2wavypurple,4dimgray
dullmagenta#2mirroredplum,3wavylime
shinyorange#noother
wavytomato#3lightsilver,5palered,1shinyaqua
lightgray#2darkteal,1poshlime,2clearfuchsia
lightchartreuse#1mutedsilver
shinycoral#1mirroredblack,5mutedsilver
drabmagenta#2mirroredorange,5darkturquoise,2vibrantteal,3stripedgold
dullbeige#2mutedturquoise,5palebronze,2plaidwhite,2dottedlavender
plaidbeige#1brightchartreuse,1vibrantred
darkorange#2clearlavender,1darkgray,4plaidchartreuse,3plaidred
lightmagenta#1plaidgray,1mirroredgray,3darkbrown,3fadedgray
poshmagenta#4cleargreen
brighttomato#1darkbrown
wavygray#2poshteal,2clearorange,3dimchartreuse,2drablime
brightblue#2wavymaroon,2palered,4dottedtan
drabolive#4vibrantblack
mutedred#4mutedplum
dullorange#4shinybronze,1shinytomato,3paleyellow,4dottedplum
poshcrimson#4mutedsalmon,2paleaqua,1poshgold
plaidlavender#4drabgray,3fadedviolet
mutedtan#4mutedcoral,1dullgray,5clearcoral
mutedplum#2shinyorange,2shinybronze,5brighttomato
palesilver#1mutedtomato,5dimcoral,3poshblack
palewhite#1mirroredlavender,2paletomato
dullgray#2plaidolive,4clearchartreuse
cleartomato#5shinypurple
dottedgray#4dimtan
vibrantolive#4brightsilver,4plaidgray,4mutedyellow,1plaidwhite
poshblack#1stripedpurple
vibrantplum#5paletomato,5dottedbeige,1drabplum,5fadedmaroon
mirroredgold#1wavylavender
wavysalmon#2fadedmaroon,2shinyred,4plaidpurple,3vibrantfuchsia
dimsalmon#2clearcoral,4dimbrown
mutedgreen#1palemagenta,5mirroredorange
dullsilver#3darkchartreuse,5brightviolet,5dullcyan
drabpurple#3vibrantred
wavyyellow#3mutedcyan,2vibrantblack,1posholive
drabsilver#3drabplum,1drabcyan
clearbeige#4clearviolet,1mutedblue,3mirroredorange
poshred#4wavyteal,3dottedsalmon
vibrantviolet#1lightgreen,2stripedcrimson
paleteal#3dullbeige,5drabolive,4brightviolet,2drabblack
brightteal#4shinyteal,4shinyturquoise,1dottedchartreuse,1wavylavender
wavyfuchsia#5clearorange,3shinytomato,2mutedturquoise
wavyteal#1clearviolet,1poshbronze,3mutedturquoise,4shinytomato
stripedplum#5lightchartreuse,5stripedwhite
stripedyellow#3poshfuchsia
clearchartreuse#1dimlavender,1poshlime,5dulllime
mirroredtan#2mirroredlavender
poshturquoise#5dimcyan,3shinyyellow,4dimbrown,5poshgray
darkcyan#4paleblack,3lightturquoise,2mirroredcyan,1wavywhite
dimmagenta#2dullgold,5brightred,4clearaqua
mirroredtomato#2poshgray,2fadedsilver
plaidviolet#4drabred,2palegray,2mutedlavender
wavyorange#4mutedcoral,3brightplum,3fadedcoral,4mutedturquoise
dimcoral#1lightred,5brightsilver,4palemagenta
dottedolive#2stripedorange
lightorange#5vibrantred,2lightturquoise,3darkcoral
fadedbrown#3plaidmaroon,1dullcoral,1drabbrown
dimblack#5vibrantviolet,3mutedblue,4poshyellow
plaidyellow#3darkcoral,1fadedbeige,3lightorange,1drablime
stripedsalmon#5mutedteal,4dottedbrown,2dimbronze,3darkblue
vibrantteal#4brightmaroon,1mutedmaroon
fadedteal#5brightbeige,1mirroredblack,5mutedlavender
stripedviolet#2stripedmagenta
mirroredgray#5lightred,2palechartreuse,5stripedgold,4palesalmon
mutedbrown#3darksalmon,4vibrantbronze,2plaidlime,3shinybronze
plaidsilver#1brightblack,2shinycyan,4drabred,2fadedindigo
shinylime#1dimpurple,3shinytomato
shinybronze#noother
drabindigo#2dimchartreuse
wavyblack#4drabblue
dottedtomato#5brighttomato,5drabtomato,1paletomato,3mirroredolive
brightturquoise#4poshyellow
cleargreen#3dottedtomato
lightbrown#5plaidfuchsia,3mirroredgray,1poshorange
clearsilver#5vibrantteal,5dimblue,4mutedgray
dullsalmon#2plaidsilver,4dimsalmon,5dimred,5vibrantmaroon
drabcoral#5palefuchsia,3dullgreen,3drabolive
dimchartreuse#4mutedmaroon
wavywhite#5mirroredturquoise
lightbronze#1stripedtomato,4fadedbronze
lightplum#5drabyellow,5drabteal,5fadedsilver,1lightmagenta
mutedmagenta#5stripedblue,3dullpurple,4vibrantred
clearviolet#noother
poshcyan#5wavybeige
lightcyan#1shinyred,4vibrantviolet,4palemagenta,4shinyturquoise
dimaqua#2plaidcrimson,2poshfuchsia
clearindigo#2darkturquoise,1stripedorange,3lightmagenta
dulllime#2mutedsilver,5fadedmaroon,4mirroredolive
dottedbeige#2darkcoral,5stripedgray,3darkaqua,3vibrantsilver
dimgray#3wavychartreuse,1mutedgreen,3shinytomato
vibrantbeige#5plaidcrimson,4drabcrimson
vibrantgray#5poshwhite,4clearbeige
vibranttomato#1stripedred,3lightmaroon,3shinyturquoise
stripedmagenta#1wavytomato,4vibrantsalmon,4brighttomato,4drabteal
darkcoral#1shinybronze,5shinytomato,4brighttomato,1plaidwhite
vibrantmaroon#1mutedcoral,2drabtomato,5mirroredolive
shinygold#5palebrown,2lightred,3drablime
paleolive#1darkturquoise
paleorange#2poshmaroon,3fadedindigo
shinyteal#4mirroredblue,2vibrantblack
dullindigo#1dullbeige,4plaidpurple
mirroredpurple#5dullgreen,3dottedcyan,1lightred,2mirroredchartreuse
dottedcoral#3vibranttan,2darkgreen
dottedsalmon#3wavyteal,1plaidbronze
fadedviolet#2plaidolive,4stripedmaroon
dottedred#1brightplum,1poshorange,4palechartreuse
drabmaroon#2shinycrimson,2dimindigo,2paleturquoise,1plaidbronze
fadedgray#2dullturquoise,3palechartreuse,2darkolive,2darkbrown
vibrantgreen#3palegreen,1clearred,2darkfuchsia
drablavender#2brightcrimson,3poshlime,2plaidlime,1vibrantfuchsia
dottedsilver#1fadedsilver
poshteal#5poshindigo
darkblack#1fadedsilver
darkyellow#1plaidviolet,2brightgray,1mirroredgold
dimfuchsia#4palechartreuse,3fadedbronze
palesalmon#3shinyorange,1darkbrown
lighttan#5palemaroon,2shinymaroon,3mutedturquoise
drabblue#5fadedteal,4dottedbeige,1dullorange
brightbeige#5mutedmaroon,2clearplum,1mirroredblack,4vibrantyellow
dimgreen#4darkfuchsia,5mutedbronze,4dimblue
mutedteal#1vibrantorange,4dottedlime,1darkturquoise,3dimplum
vibrantwhite#4dullred,3stripedwhite
shinysilver#1dimteal,3palemaroon,2plaidolive
palecyan#3dimturquoise
fadedplum#2palepurple,2darkwhite,4dottedchartreuse,5vibrantmaroon
dottedlavender#1shinytomato,5stripedgold,5palebrown
brightwhite#1fadedmagenta,2mirroredsalmon,5brightsalmon
darklavender#1poshteal
poshgray#5vibrantyellow,5dottedsalmon,3shinytomato,2clearbeige
darkgray#3stripedgray,3wavytan
darklime#1darksalmon,5palegreen,2stripedgreen,5mirroredblue
vibrantblack#3lightred,4plaidlime,3poshlime,2dottedlavender
lightbeige#5mutedblue,2fadedred,1mutedturquoise
darkred#2fadedcrimson,1wavymaroon,2clearindigo
shinycrimson#1mutedred,5shinybronze,1plaidgreen,5mirroredorange
dimpurple#5stripedpurple,2vibrantbronze,2stripedbeige
wavymaroon#1fadedsilver,4mutedblack
dullblue#2brightgold,3palesalmon
plaidgreen#4darkbrown,1shinyorange
"########;
    input.to_string()
}

