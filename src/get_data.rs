use std::{fs::File, io::Read, path::Path};

use color_eyre::eyre;

pub async fn get_json_data(json_file: String) -> color_eyre::Result<String> {
    // Ensure that the specified data file has the correct extension.
    if !json_file.to_lowercase().ends_with(".json") {
        return Err(eyre::eyre!(
            "Incorrect JSON file extension for data file '{}'.",
            json_file
        ));
    }

    let json_str = match json_file.to_lowercase().starts_with("http") {
        true => {
            // Reference is to a URL.
            let response = reqwest::get(&json_file).await?;

            if !response.status().is_success() {
                return Err(eyre::eyre!(
                    "Bad response while accessing data file URL '{}'.",
                    &json_file
                ));
            }

            let site_data: String = response.text().await.map_err(|error| {
                eyre::eyre!(
                    "Problem parsing JSON contents at '{}': {}.",
                    json_file,
                    error
                )
            })?;

            site_data
        }
        false => {
            // Reference is to a file.
            let path = Path::new(&json_file);

            let mut file = File::open(path).map_err(|_| {
                eyre::eyre!(
                    "Problem while attempting to access data file '{}'.",
                    json_file
                )
            })?;

            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            contents
        }
    };

    Ok(json_str)
}

/// the default sites to check for sherlock locally
/// includes >400 websites and their error messages
pub fn get_default_data() -> String {
    r#"
{
  "$schema": "data.schema.json",
  "1337x": {
    "errorMsg": [
      "<title>Error something went wrong.</title>",
      "<head><title>404 Not Found</title></head>"
    ],
    "errorType": "message",
    "regexCheck": "^[A-Za-z0-9]{4,12}$",
    "url": "https://www.1337x.to/user/{}/",
    "urlMain": "https://www.1337x.to/",
    "username_claimed": "FitGirl"
  },
  "2Dimensions": {
    "errorType": "status_code",
    "url": "https://2Dimensions.com/a/{}",
    "urlMain": "https://2Dimensions.com/",
    "username_claimed": "blue"
  },
  "3dnews": {
    "errorMsg": "\u041f\u043e\u043b\u044c\u0437\u043e\u0432\u0430\u0442\u0435\u043b\u044c \u043d\u0435 \u0437\u0430\u0440\u0435\u0433\u0438\u0441\u0442\u0440\u0438\u0440\u043e\u0432\u0430\u043d \u0438 \u043d\u0435 \u0438\u043c\u0435\u0435\u0442 \u043f\u0440\u043e\u0444\u0438\u043b\u044f \u0434\u043b\u044f \u043f\u0440\u043e\u0441\u043c\u043e\u0442\u0440\u0430.",
    "errorType": "message",
    "url": "http://forum.3dnews.ru/member.php?username={}",
    "urlMain": "http://forum.3dnews.ru/",
    "username_claimed": "red"
  },
  "7Cups": {
    "errorType": "status_code",
    "url": "https://www.7cups.com/@{}",
    "urlMain": "https://www.7cups.com/",
    "username_claimed": "blue"
  },
  "8tracks": {
    "errorMsg": "This page has vanished",
    "errorType": "message",
    "url": "https://8tracks.com/{}",
    "urlMain": "https://8tracks.com/",
    "username_claimed": "blue"
  },
  "9GAG": {
    "errorType": "status_code",
    "url": "https://www.9gag.com/u/{}",
    "urlMain": "https://www.9gag.com/",
    "username_claimed": "blue"
  },
  "APClips": {
    "errorMsg": "Amateur Porn Content Creators",
    "errorType": "message",
    "isNSFW": true,
    "url": "https://apclips.com/{}",
    "urlMain": "https://apclips.com/",
    "username_claimed": "onlybbyraq"
  },
  "About.me": {
    "errorType": "status_code",
    "url": "https://about.me/{}",
    "urlMain": "https://about.me/",
    "username_claimed": "blue"
  },
  "Academia.edu": {
    "errorType": "status_code",
    "regexCheck": "^[^.]*$",
    "url": "https://independent.academia.edu/{}",
    "urlMain": "https://www.academia.edu/",
    "username_claimed": "blue"
  },
  "AdmireMe.Vip": {
    "errorMsg": "Page Not Found",
    "errorType": "message",
    "isNSFW": true,
    "url": "https://admireme.vip/{}",
    "urlMain": "https://admireme.vip/",
    "username_claimed": "DemiDevil"
  },
  "Air Pilot Life": {
    "errorMsg": "Oops! That page doesn\u2019t exist or is private",
    "errorType": "message",
    "url": "https://airlinepilot.life/u/{}",
    "urlMain": "https://airlinepilot.life/",
    "username_claimed": "chris"
  },
  "Airbit": {
    "errorType": "status_code",
    "url": "https://airbit.com/{}",
    "urlMain": "https://airbit.com/",
    "username_claimed": "airbit"
  },
  "Airliners": {
    "errorType": "status_code",
    "url": "https://www.airliners.net/user/{}/profile/photos",
    "urlMain": "https://www.airliners.net/",
    "username_claimed": "yushinlin"
  },
  "All Things Worn": {
    "errorMsg": "Sell Used Panties",
    "errorType": "message",
    "isNSFW": true,
    "url": "https://www.allthingsworn.com/profile/{}",
    "urlMain": "https://www.allthingsworn.com",
    "username_claimed": "pink"
  },
  "AllMyLinks": {
    "errorMsg": "Not Found",
    "errorType": "message",
    "regexCheck": "^[a-z0-9][a-z0-9-]{2,32}$",
    "url": "https://allmylinks.com/{}",
    "urlMain": "https://allmylinks.com/",
    "username_claimed": "blue"
  },
  "Amino": {
    "errorType": "status_code",
    "url": "https://aminoapps.com/u/{}",
    "urlMain": "https://aminoapps.com",
    "username_claimed": "blue"
  },
  "AniWorld": {
    "errorMsg": "Dieses Profil ist nicht verf\u00fcgbar",
    "errorType": "message",
    "url": "https://aniworld.to/user/profil/{}",
    "urlMain": "https://aniworld.to/",
    "username_claimed": "blue"
  },
  "Anilist": {
    "errorType": "status_code",
    "regexCheck": "^[A-Za-z0-9]{2,20}$",
    "request_method": "POST",
    "request_payload": {
      "query": "query($name:String){User(name:$name){id}}",
      "variables": {
        "name": "{}"
      }
    },
    "url": "https://anilist.co/user/{}/",
    "urlMain": "https://anilist.co/",
    "urlProbe": "https://graphql.anilist.co/",
    "username_claimed": "Josh"
  },
  "Apple Developer": {
    "errorType": "status_code",
    "url": "https://developer.apple.com/forums/profile/{}",
    "urlMain": "https://developer.apple.com",
    "username_claimed": "lio24d"
  },
  "Apple Discussions": {
    "errorMsg": "The page you tried was not found. You may have used an outdated link or may have typed the address (URL) incorrectly.",
    "errorType": "message",
    "url": "https://discussions.apple.com/profile/{}",
    "urlMain": "https://discussions.apple.com",
    "username_claimed": "jason"
  },
  "Archive of Our Own": {
    "errorType": "status_code",
    "regexCheck": "^[^.]*?$",
    "url": "https://archiveofourown.org/users/{}",
    "urlMain": "https://archiveofourown.org/",
    "username_claimed": "test"
  },
  "Archive.org": {
    "__comment__": "'The resource could not be found' relates to archive downtime",
    "errorMsg": [
      "could not fetch an account with user item identifier",
      "The resource could not be found"
    ],
    "errorType": "message",
    "url": "https://archive.org/details/@{}",
    "urlMain": "https://archive.org",
    "urlProbe": "https://archive.org/details/@{}?noscript=true",
    "username_claimed": "blue"
  },
  "ArtStation": {
    "errorType": "status_code",
    "url": "https://www.artstation.com/{}",
    "urlMain": "https://www.artstation.com/",
    "username_claimed": "Blue"
  },
  "Asciinema": {
    "errorType": "status_code",
    "url": "https://asciinema.org/~{}",
    "urlMain": "https://asciinema.org",
    "username_claimed": "red"
  },
  "Ask Fedora": {
    "errorType": "status_code",
    "url": "https://ask.fedoraproject.org/u/{}",
    "urlMain": "https://ask.fedoraproject.org/",
    "username_claimed": "red"
  },
  "AskFM": {
    "errorMsg": "Well, apparently not anymore.",
    "errorType": "message",
    "regexCheck": "^[a-zA-Z0-9_]{3,40}$",
    "url": "https://ask.fm/{}",
    "urlMain": "https://ask.fm/",
    "username_claimed": "blue"
  },
  "Audiojungle": {
    "errorType": "status_code",
    "regexCheck": "^[a-zA-Z0-9_]+$",
    "url": "https://audiojungle.net/user/{}",
    "urlMain": "https://audiojungle.net/",
    "username_claimed": "blue"
  },
  "Autofrage": {
    "errorType": "status_code",
    "url": "https://www.autofrage.net/nutzer/{}",
    "urlMain": "https://www.autofrage.net/",
    "username_claimed": "autofrage"
  },
  "Avizo": {
    "errorType": "response_url",
    "errorUrl": "https://www.avizo.cz/",
    "url": "https://www.avizo.cz/{}/",
    "urlMain": "https://www.avizo.cz/",
    "username_claimed": "blue"
  },
  "BLIP.fm": {
    "errorType": "status_code",
    "regexCheck": "^[a-zA-Z0-9_]{1,30}$",
    "url": "https://blip.fm/{}",
    "urlMain": "https://blip.fm/",
    "username_claimed": "blue"
  },
  "BOOTH": {
    "errorType": "response_url",
    "errorUrl": "https://booth.pm/",
    "regexCheck": "^[a-zA-Z0-9@_-]$",
    "url": "https://{}.booth.pm/",
    "urlMain": "https://booth.pm/",
    "username_claimed": "blue"
  },
  "Bandcamp": {
    "errorType": "status_code",
    "url": "https://www.bandcamp.com/{}",
    "urlMain": "https://www.bandcamp.com/",
    "username_claimed": "blue"
  },
  "Bazar.cz": {
    "errorType": "response_url",
    "errorUrl": "https://www.bazar.cz/error404.aspx",
    "url": "https://www.bazar.cz/{}/",
    "urlMain": "https://www.bazar.cz/",
    "username_claimed": "pianina"
  },
  "Behance": {
    "errorType": "status_code",
    "url": "https://www.behance.net/{}",
    "urlMain": "https://www.behance.net/",
    "username_claimed": "blue"
  },
  "Bezuzyteczna": {
    "errorType": "status_code",
    "url": "https://bezuzyteczna.pl/uzytkownicy/{}",
    "urlMain": "https://bezuzyteczna.pl",
    "username_claimed": "Jackson"
  },
  "BiggerPockets": {
    "errorType": "status_code",
    "url": "https://www.biggerpockets.com/users/{}",
    "urlMain": "https://www.biggerpockets.com/",
    "username_claimed": "blue"
  },
  "Bikemap": {
    "errorType": "status_code",
    "url": "https://www.bikemap.net/en/u/{}/routes/created/",
    "urlMain": "https://www.bikemap.net/",
    "username_claimed": "bikemap"
  },
  "BioHacking": {
    "errorType": "status_code",
    "url": "https://forum.dangerousthings.com/u/{}",
    "urlMain": "https://forum.dangerousthings.com/",
    "username_claimed": "blue"
  },
  "BitBucket": {
    "errorType": "status_code",
    "regexCheck": "^[a-zA-Z0-9-_]{1,30}$",
    "url": "https://bitbucket.org/{}/",
    "urlMain": "https://bitbucket.org/",
    "username_claimed": "white"
  },
  "Bitwarden Forum": {
    "errorType": "status_code",
    "regexCheck": "^(?![.-])[a-zA-Z0-9_.-]{3,20}$",
    "url": "https://community.bitwarden.com/u/{}/summary",
    "urlMain": "https://bitwarden.com/",
    "username_claimed": "blue"
  },
  "Blipfoto": {
    "errorType": "status_code",
    "url": "https://www.blipfoto.com/{}",
    "urlMain": "https://www.blipfoto.com/",
    "username_claimed": "blue"
  },
  "Blogger": {
    "errorType": "status_code",
    "regexCheck": "^[a-zA-Z][a-zA-Z0-9_-]*$",
    "url": "https://{}.blogspot.com",
    "urlMain": "https://www.blogger.com/",
    "username_claimed": "blue"
  },
  "BodyBuilding": {
    "errorType": "response_url",
    "errorUrl": "https://bodyspace.bodybuilding.com/",
    "url": "https://bodyspace.bodybuilding.com/{}",
    "urlMain": "https://bodyspace.bodybuilding.com/",
    "username_claimed": "blue"
  },
  "BongaCams": {
    "errorType": "status_code",
    "isNSFW": true,
    "url": "https://pt.bongacams.com/profile/{}",
    "urlMain": "https://pt.bongacams.com",
    "username_claimed": "asuna-black"
  },
  "Bookcrossing": {
    "errorType": "status_code",
    "url": "https://www.bookcrossing.com/mybookshelf/{}/",
    "urlMain": "https://www.bookcrossing.com/",
    "username_claimed": "blue"
  },
  "BraveCommunity": {
    "errorType": "status_code",
    "url": "https://community.brave.com/u/{}/",
    "urlMain": "https://community.brave.com/",
    "username_claimed": "blue"
  },
  "BugCrowd": {
    "errorType": "status_code",
    "url": "https://bugcrowd.com/{}",
    "urlMain": "https://bugcrowd.com/",
    "username_claimed": "ppfeister"
  },
  "BuyMeACoffee": {
    "errorType": "status_code",
    "regexCheck": "[a-zA-Z0-9]{3,15}",
    "url": "https://buymeacoff.ee/{}",
    "urlMain": "https://www.buymeacoffee.com/",
    "urlProbe": "https://www.buymeacoffee.com/{}",
    "username_claimed": "red"
  },
  "BuzzFeed": {
    "errorType": "status_code",
    "url": "https://buzzfeed.com/{}",
    "urlMain": "https://buzzfeed.com/",
    "username_claimed": "blue"
  },
  "CGTrader": {
    "errorType": "status_code",
    "regexCheck": "^[^.]*?$",
    "url": "https://www.cgtrader.com/{}",
    "urlMain": "https://www.cgtrader.com",
    "username_claimed": "blue"
  },
  "CNET": {
    "errorType": "status_code",
    "regexCheck": "^[a-z].*$",
    "url": "https://www.cnet.com/profiles/{}/",
    "urlMain": "https://www.cnet.com/",
    "username_claimed": "melliott"
  },
  "CSSBattle": {
    "errorType": "status_code",
    "url": "https://cssbattle.dev/player/{}",
    "urlMain": "https://cssbattle.dev",
    "username_claimed": "beo"
  },
  "CTAN": {
    "errorType": "status_code",
    "url": "https://ctan.org/author/{}",
    "urlMain": "https://ctan.org/",
    "username_claimed": "briggs"
  },
  "Caddy Community": {
    "errorType": "status_code",
    "url": "https://caddy.community/u/{}/summary",
    "urlMain": "https://caddy.community/",
    "username_claimed": "taako_magnusen"
  },
  "Car Talk Community": {
    "errorType": "status_code",
    "url": "https://community.cartalk.com/u/{}/summary",
    "urlMain": "https://community.cartalk.com/",
    "username_claimed": "always_fixing"
  },
  "Carbonmade": {
    "errorType": "response_url",
    "errorUrl": "https://carbonmade.com/fourohfour?domain={}.carbonmade.com",
    "regexCheck": "^[a-zA-Z0-9@_-]$",
    "url": "https://{}.carbonmade.com",
    "urlMain": "https://carbonmade.com/",
    "username_claimed": "jenny"
  },
  "Career.habr": {
    "errorMsg": "<h1>\u041e\u0448\u0438\u0431\u043a\u0430 404</h1>",
    "errorType": "message",
    "url": "https://career.habr.com/{}",
    "urlMain": "https://career.habr.com/",
    "username_claimed": "blue"
  },
  "Championat": {
    "errorType": "status_code",
    "url": "https://www.championat.com/user/{}",
    "urlMain": "https://www.championat.com/",
    "username_claimed": "blue"
  },
  "Chaos": {
    "errorType": "status_code",
    "url": "https://chaos.social/@{}",
    "urlMain": "https://chaos.social/",
    "username_claimed": "ordnung"
  },
  "Chatujme.cz": {
    "errorMsg": "Neexistujic\u00ed profil",
    "errorType": "message",
    "regexCheck": "^[a-zA-Z][a-zA-Z1-9_-]*$",
    "url": "https://profil.chatujme.cz/{}",
    "urlMain": "https://chatujme.cz/",
    "username_claimed": "david"
  },
  "ChaturBate": {
    "errorType": "status_code",
    "isNSFW": true,
    "url": "https://chaturbate.com/{}",
    "urlMain": "https://chaturbate.com",
    "username_claimed": "cute18cute"
  },
  "Chess": {
    "errorMsg": "Username is valid",
    "errorType": "message",
    "regexCheck": "^[a-z1-9]{3,25}$",
    "url": "https://www.chess.com/member/{}",
    "urlMain": "https://www.chess.com/",
    "urlProbe": "https://www.chess.com/callback/user/valid?username={}",
    "username_claimed": "blue"
  },
  "Choice Community": {
    "errorType": "status_code",
    "url": "https://choice.community/u/{}/summary",
    "urlMain": "https://choice.community/",
    "username_claimed": "gordon"
  },
  "Clapper": {
    "errorType": "status_code",
    "url": "https://clapperapp.com/{}",
    "urlMain": "https://clapperapp.com/",
    "username_claimed": "blue"
  },
  "CloudflareCommunity": {
    "errorType": "status_code",
    "url": "https://community.cloudflare.com/u/{}",
    "urlMain": "https://community.cloudflare.com/",
    "username_claimed": "blue"
  },
  "Clozemaster": {
    "errorMsg": "Oh no! Player not found.",
    "errorType": "message",
    "url": "https://www.clozemaster.com/players/{}",
    "urlMain": "https://www.clozemaster.com",
    "username_claimed": "green"
  },
  "Clubhouse": {
    "errorType": "status_code",
    "url": "https://www.clubhouse.com/@{}",
    "urlMain": "https://www.clubhouse.com",
    "username_claimed": "waniathar"
  },
  "Code Snippet Wiki": {
    "errorMsg": "This user has not filled out their profile page yet",
    "errorType": "message",
    "url": "https://codesnippets.fandom.com/wiki/User:{}",
    "urlMain": "https://codesnippets.fandom.com",
    "username_claimed": "bob"
  },
  "Codeberg": {
    "errorType": "status_code",
    "url": "https://codeberg.org/{}",
    "urlMain": "https://codeberg.org/",
    "username_claimed": "blue"
  },
  "Codecademy": {
    "errorMsg": "This profile could not be found",
    "errorType": "message",
    "url": "https://www.codecademy.com/profiles/{}",
    "urlMain": "https://www.codecademy.com/",
    "username_claimed": "blue"
  },
  "Codechef": {
    "errorType": "response_url",
    "errorUrl": "https://www.codechef.com/",
    "url": "https://www.codechef.com/users/{}",
    "urlMain": "https://www.codechef.com/",
    "username_claimed": "blue"
  },
  "Codeforces": {
    "errorType": "status_code",
    "url": "https://codeforces.com/profile/{}",
    "urlMain": "https://codeforces.com/",
    "urlProbe": "https://codeforces.com/api/user.info?handles={}",
    "username_claimed": "tourist"
  },
  "Codepen": {
    "errorType": "status_code",
    "url": "https://codepen.io/{}",
    "urlMain": "https://codepen.io/",
    "username_claimed": "blue"
  },
  "Coders Rank": {
    "errorMsg": "not a registered member",
    "errorType": "message",
    "regexCheck": "^[a-zA-Z0-9](?:[a-zA-Z0-9]|-(?=[a-zA-Z0-9])){0,38}$",
    "url": "https://profile.codersrank.io/user/{}/",
    "urlMain": "https://codersrank.io/",
    "username_claimed": "rootkit7628"
  },
  "Coderwall": {
    "errorType": "status_code",
    "url": "https://coderwall.com/{}",
    "urlMain": "https://coderwall.com",
    "username_claimed": "hacker"
  },
  "Codewars": {
    "errorType": "status_code",
    "url": "https://www.codewars.com/users/{}",
    "urlMain": "https://www.codewars.com",
    "username_claimed": "example"
  },
  "Coinvote": {
    "errorType": "status_code",
    "url": "https://coinvote.cc/profile/{}",
    "urlMain": "https://coinvote.cc/",
    "username_claimed": "blue"
  },
  "ColourLovers": {
    "errorType": "status_code",
    "url": "https://www.colourlovers.com/lover/{}",
    "urlMain": "https://www.colourlovers.com/",
    "username_claimed": "blue"
  },
  "Contently": {
    "errorType": "response_url",
    "errorUrl": "https://contently.com",
    "regexCheck": "^[a-zA-Z][a-zA-Z0-9_-]*$",
    "url": "https://{}.contently.com/",
    "urlMain": "https://contently.com/",
    "username_claimed": "jordanteicher"
  },
  "Coroflot": {
    "errorType": "status_code",
    "url": "https://www.coroflot.com/{}",
    "urlMain": "https://coroflot.com/",
    "username_claimed": "blue"
  },
  "Cracked": {
    "errorType": "response_url",
    "errorUrl": "https://www.cracked.com/",
    "url": "https://www.cracked.com/members/{}/",
    "urlMain": "https://www.cracked.com/",
    "username_claimed": "blue"
  },
  "Crevado": {
    "errorType": "status_code",
    "regexCheck": "^[a-zA-Z0-9@_-]$",
    "url": "https://{}.crevado.com",
    "urlMain": "https://crevado.com/",
    "username_claimed": "blue"
  },
  "Crowdin": {
    "errorType": "status_code",
    "regexCheck": "^[a-zA-Z0-9._-]{2,255}$",
    "url": "https://crowdin.com/profile/{}",
    "urlMain": "https://crowdin.com/",
    "username_claimed": "blue"
  },
  "Cryptomator Forum": {
    "errorType": "status_code",
    "url": "https://community.cryptomator.org/u/{}",
    "urlMain": "https://community.cryptomator.org/",
    "username_claimed": "michael"
  },
  "Cults3D": {
    "errorMsg": "Oh dear, this page is not working!",
    "errorType": "message",
    "url": "https://cults3d.com/en/users/{}/creations",
    "urlMain": "https://cults3d.com/en",
    "username_claimed": "brown"
  },
  "CyberDefenders": {
    "errorMsg": "<title>Blue Team Training for SOC analysts and DFIR - CyberDefenders</title>",
    "errorType": "message",
    "regexCheck": "^[^\\/:*?\"<>|@]{3,50}$",
    "request_method": "GET",
    "url": "https://cyberdefenders.org/p/{}",
    "urlMain": "https://cyberdefenders.org/",
    "username_claimed": "mlohn"
  },
  "DEV Community": {
    "errorType": "status_code",
    "regexCheck": "^[a-zA-Z][a-zA-Z0-9_-]*$",
    "url": "https://dev.to/{}",
    "urlMain": "https://dev.to/",
    "username_claimed": "blue"
  },
  "DMOJ": {
    "errorMsg": "No such user",
    "errorType": "message",
    "url": "https://dmoj.ca/user/{}",
    "urlMain": "https://dmoj.ca/",
    "username_claimed": "junferno"
  },
  "DailyMotion": {
    "errorType": "status_code",
    "url": "https://www.dailymotion.com/{}",
    "urlMain": "https://www.dailymotion.com/",
    "username_claimed": "blue"
  },
  "Dealabs": {
    "errorMsg": "La page que vous essayez",
    "errorType": "message",
    "regexCheck": "[a-z0-9]{4,16}",
    "url": "https://www.dealabs.com/profile/{}",
    "urlMain": "https://www.dealabs.com/",
    "username_claimed": "blue"
  },
  "DeviantART": {
    "errorType": "status_code",
    "regexCheck": "^[a-zA-Z][a-zA-Z0-9_-]*$",
    "url": "https://{}.deviantart.com",
    "urlMain": "https://deviantart.com",
    "username_claimed": "blue"
  },
  "Discogs": {
    "errorType": "status_code",
    "url": "https://www.discogs.com/user/{}",
    "urlMain": "https://www.discogs.com/",
    "username_claimed": "blue"
  },
  "Discord": {
    "errorType": "message",
    "url": "https://discord.com",
    "urlMain": "https://discord.com/",
    "urlProbe": "https://discord.com/api/v9/unique-username/username-attempt-unauthed",
    "errorMsg": [
      "{\"taken\":false}",
      "The resource is being rate limited"
    ],
    "request_method": "POST",
    "request_payload": {
      "username": "{}"
    },
    "headers": {
      "Content-Type": "application/json"
    },
    "username_claimed": "blue"
  },
  "Discuss.Elastic.co": {
    "errorType": "status_code",
    "url": "https://discuss.elastic.co/u/{}",
    "urlMain": "https://discuss.elastic.co/",
    "username_claimed": "blue"
  },
  "Disqus": {
    "errorType": "status_code",
    "url": "https://disqus.com/{}",
    "urlMain": "https://disqus.com/",
    "username_claimed": "blue"
  },
  "Docker Hub": {
    "errorType": "status_code",
    "url": "https://hub.docker.com/u/{}/",
    "urlMain": "https://hub.docker.com/",
    "urlProbe": "https://hub.docker.com/v2/users/{}/",
    "username_claimed": "blue"
  },
  "Dribbble": {
    "errorMsg": "Whoops, that page is gone.",
    "errorType": "message",
    "regexCheck": "^[a-zA-Z][a-zA-Z0-9_-]*$",
    "url": "https://dribbble.com/{}",
    "urlMain": "https://dribbble.com/",
    "username_claimed": "blue"
  },
  "Duolingo": {
    "errorMsg": "{\"users\":[]}",
    "errorType": "message",
    "headers": {
      "User-Agent": "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/116.0"
    },
    "url": "https://www.duolingo.com/profile/{}",
    "urlMain": "https://duolingo.com/",
    "urlProbe": "https://www.duolingo.com/2017-06-30/users?username={}",
    "username_claimed": "blue"
  },
  "Eintracht Frankfurt Forum": {
    "errorType": "status_code",
    "regexCheck": "^[^.]*?$",
    "url": "https://community.eintracht.de/fans/{}",
    "urlMain": "https://community.eintracht.de/",
    "username_claimed": "mmammu"
  },
  "Envato Forum": {
    "errorType": "status_code",
    "url": "https://forums.envato.com/u/{}",
    "urlMain": "https://forums.envato.com/",
    "username_claimed": "enabled"
  },
  "Erome": {
    "errorType": "status_code",
    "isNSFW": true,
    "url": "https://www.erome.com/{}",
    "urlMain": "https://www.erome.com/",
    "username_claimed": "bob"
  },
  "Exposure": {
    "errorType": "status_code",
    "url": "https://{}.exposure.co/",
    "urlMain": "https://exposure.co/",
    "username_claimed": "jonasjacobsson"
  },
  "EyeEm": {
    "errorType": "status_code",
    "url": "https://www.eyeem.com/u/{}",
    "urlMain": "https://www.eyeem.com/",
    "username_claimed": "blue"
  },
  "F3.cool": {
    "errorType": "status_code",
    "url": "https://f3.cool/{}/",
    "urlMain": "https://f3.cool/",
    "username_claimed": "blue"
  },
  "Fameswap": {
    "errorType": "status_code",
    "url": "https://fameswap.com/user/{}",
    "urlMain": "https://fameswap.com/",
    "username_claimed": "fameswap"
  },
  "Fandom": {
    "errorType": "status_code",
    "url": "https://www.fandom.com/u/{}",
    "urlMain": "https://www.fandom.com/",
    "username_claimed": "Jungypoo"
  },
  "Finanzfrage": {
    "errorType": "status_code",
    "url": "https://www.finanzfrage.net/nutzer/{}",
    "urlMain": "https://www.finanzfrage.net/",
    "username_claimed": "finanzfrage"
  },
  "Fiverr": {
    "errorMsg": "\"status\":\"success\"",
    "errorType": "message",
    "regexCheck": "^[A-Za-z][A-Za-z\\d_]{5,14}$",
    "request_method": "POST",
    "request_payload": {
      "username": "{}"
    },
    "url": "https://www.fiverr.com/{}",
    "urlMain": "https://www.fiverr.com/",
    "urlProbe": "https://www.fiverr.com/validate_username",
    "username_claimed": "blueman"
  },
  "Flickr": {
    "errorType": "status_code",
    "url": "https://www.flickr.com/people/{}",
    "urlMain": "https://www.flickr.com/",
    "username_claimed": "blue"
  },
  "Flightradar24": {
    "errorType": "status_code",
    "regexCheck": "^[a-zA-Z0-9_]{3,20}$",
    "url": "https://my.flightradar24.com/{}",
    "urlMain": "https://www.flightradar24.com/",
    "username_claimed": "jebbrooks"
  },
  "Flipboard": {
    "errorType": "status_code",
    "regexCheck": "^([a-zA-Z0-9_]){1,15}$",
    "url": "https://flipboard.com/@{}",
    "urlMain": "https://flipboard.com/",
    "username_claimed": "blue"
  },
  "Football": {
    "errorMsg": "\u041f\u043e\u043b\u044c\u0437\u043e\u0432\u0430\u0442\u0435\u043b\u044c \u0441 \u0442\u0430\u043a\u0438\u043c \u0438\u043c\u0435\u043d\u0435\u043c \u043d\u0435 \u043d\u0430\u0439\u0434\u0435\u043d",
    "errorType": "message",
    "url": "https://www.rusfootball.info/user/{}/",
    "urlMain": "https://www.rusfootball.info/",
    "username_claimed": "solo87"
  },
  "FortniteTracker": {
    "errorType": "status_code",
    "url": "https://fortnitetracker.com/profile/all/{}",
    "urlMain": "https://fortnitetracker.com/challenges",
    "username_claimed": "blue"
  },
  "Forum Ophilia": {
    "errorMsg": "that user does not exist",
    "errorType": "message",
    "isNSFW": true,
    "url": "https://www.forumophilia.com/profile.php?mode=viewprofile&u={}",
    "urlMain": "https://www.forumophilia.com/",
    "username_claimed": "bob"
  },
  "Fosstodon": {
    "errorType": "status_code",
    "regexCheck": "^[a-zA-Z0-9_]{1,30}$",
    "url": "https://fosstodon.org/@{}",
    "urlMain": "https://fosstodon.org/",
    "username_claimed": "blue"
  },
  "Freelance.habr": {
    "errorMsg": "<div class=\"icon_user_locked\"></div>",
    "errorType": "message",
    "regexCheck": "^((?!\\.).)*$",
    "url": "https://freelance.habr.com/freelancers/{}",
    "urlMain": "https://freelance.habr.com/",
    "username_claimed": "adam"
  },
  "Freelancer": {
    "errorMsg": "\"users\":{}",
    "errorType": "message",
    "url": "https://www.freelancer.com/u/{}",
    "urlMain": "https://www.freelancer.com/",
    "urlProbe": "https://www.freelancer.com/api/users/0.1/users?usernames%5B%5D={}&compact=true",
    "username_claimed": "red0xff"
  },
  "Freesound": {
    "errorType": "status_code",
    "url": "https://freesound.org/people/{}/",
    "urlMain": "https://freesound.org/",
    "username_claimed": "blue"
  },
  "GNOME VCS": {
    "errorType": "response_url",
    "errorUrl": "https://gitlab.gnome.org/{}",
    "regexCheck": "^(?!-)[a-zA-Z0-9_.-]{2,255}(?<!\\.)$",
    "url": "https://gitlab.gnome.org/{}",
    "urlMain": "https://gitlab.gnome.org/",
    "username_claimed": "adam"
  },
  "GaiaOnline": {
    "errorMsg": "No user ID specified or user does not exist",
    "errorType": "message",
    "url": "https://www.gaiaonline.com/profiles/{}",
    "urlMain": "https://www.gaiaonline.com/",
    "username_claimed": "adam"
  },
  "Gamespot": {
    "errorType": "status_code",
    "url": "https://www.gamespot.com/profile/{}/",
    "urlMain": "https://www.gamespot.com/",
    "username_claimed": "blue"
  },
  "GeeksforGeeks": {
    "errorType": "status_code",
    "url": "https://auth.geeksforgeeks.org/user/{}",
    "urlMain": "https://www.geeksforgeeks.org/",
    "username_claimed": "adam"
  },
  "Genius (Artists)": {
    "errorType": "status_code",
    "regexCheck": "^[a-zA-Z0-9]{5,50}$",
    "url": "https://genius.com/artists/{}",
    "urlMain": "https://genius.com/",
    "username_claimed": "genius"
  },
  "Genius (Users)": {
    "errorType": "status_code",
    "regexCheck": "^[a-zA-Z0-9]*?$",
    "url": "https://genius.com/{}",
    "urlMain": "https://genius.com/",
    "username_claimed": "genius"
  },
  "Gesundheitsfrage": {
    "errorType": "status_code",
    "url": "https://www.gesundheitsfrage.net/nutzer/{}",
    "urlMain": "https://www.gesundheitsfrage.net/",
    "username_claimed": "gutefrage"
  },
  "GetMyUni": {
    "errorType": "status_code",
    "url": "https://www.getmyuni.com/user/{}",
    "urlMain": "https://getmyuni.com/",
    "username_claimed": "Upneet.Grover17"
  },
  "Giant Bomb": {
    "errorType": "status_code",
    "url": "https://www.giantbomb.com/profile/{}/",
    "urlMain": "https://www.giantbomb.com/",
    "username_claimed": "bob"
  },
  "Giphy": {
    "errorType": "status_code",
    "url": "https://giphy.com/{}",
    "urlMain": "https://giphy.com/",
    "username_claimed": "blue"
  },
  "GitBook": {
    "errorType": "status_code",
    "regexCheck": "^[a-zA-Z0-9@_-]$",
    "url": "https://{}.gitbook.io/",
    "urlMain": "https://gitbook.com/",
    "username_claimed": "gitbook"
  },
  "GitHub": {
    "errorType": "status_code",
    "regexCheck": "^[a-zA-Z0-9](?:[a-zA-Z0-9]|-(?=[a-zA-Z0-9])){0,38}$",
    "url": "https://www.github.com/{}",
    "urlMain": "https://www.github.com/",
    "username_claimed": "blue"
  },
  "GitLab": {
    "errorMsg": "[]",
    "errorType": "message",
    "url": "https://gitlab.com/{}",
    "urlMain": "https://gitlab.com/",
    "urlProbe": "https://gitlab.com/api/v4/users?username={}",
    "username_claimed": "blue"
  },
  "Gitee": {
    "errorType": "status_code",
    "url": "https://gitee.com/{}",
    "urlMain": "https://gitee.com/",
    "username_claimed": "wizzer"
  },
  "GoodReads": {
    "errorType": "status_code",
    "url": "https://www.goodreads.com/{}",
    "urlMain": "https://www.goodreads.com/",
    "username_claimed": "blue"
  },
  "Google Play": {
    "errorMsg": "the requested URL was not found on this server",
    "errorType": "message",
    "url": "https://play.google.com/store/apps/developer?id={}",
    "urlMain": "https://play.google.com",
    "username_claimed": "GitHub"
  },
  "Gradle": {
    "errorType": "status_code",
    "regexCheck": "^(?!-)[a-zA-Z0-9-]{3,}(?<!-)$",
    "url": "https://plugins.gradle.org/u/{}",
    "urlMain": "https://gradle.org/",
    "username_claimed": "jetbrains"
  },
  "Grailed": {
    "errorType": "response_url",
    "errorUrl": "https://www.grailed.com/{}",
    "url": "https://www.grailed.com/{}",
    "urlMain": "https://www.grailed.com/",
    "username_claimed": "blue"
  },
  "Gravatar": {
    "errorType": "status_code",
    "regexCheck": "^((?!\\.).)*$",
    "url": "http://en.gravatar.com/{}",
    "urlMain": "http://en.gravatar.com/",
    "username_claimed": "blue"
  },
  "Gumroad": {
    "errorMsg": "Page not found (404) - Gumroad",
    "errorType": "message",
    "regexCheck": "^[^.]*?$",
    "url": "https://www.gumroad.com/{}",
    "urlMain": "https://www.gumroad.com/",
    "username_claimed": "blue"
  },
  "Gutefrage": {
    "errorType": "status_code",
    "url": "https://www.gutefrage.net/nutzer/{}",
    "urlMain": "https://www.gutefrage.net/",
    "username_claimed": "gutefrage"
  },
  "HackTheBox": {
    "errorType": "status_code",
    "url": "https://forum.hackthebox.eu/profile/{}",
    "urlMain": "https://forum.hackthebox.eu/",
    "username_claimed": "angar"
  },
  "Hackaday": {
    "errorType": "status_code",
    "url": "https://hackaday.io/{}",
    "urlMain": "https://hackaday.io/",
    "username_claimed": "adam"
  },
  "HackenProof (Hackers)": {
    "errorMsg": "<title>Web3\u2019s Largest Ethical Hackers Community | HackenProof</title>",
    "errorType": "message",
    "regexCheck": "^[\\w-]{,34}$",
    "url": "https://hackenproof.com/hackers/{}",
    "urlMain": "https://hackenproof.com/",
    "username_claimed": "blazezaria"
  },
  "HackerEarth": {
    "errorMsg": "404. URL not found.",
    "errorType": "message",
    "url": "https://hackerearth.com/@{}",
    "urlMain": "https://hackerearth.com/",
    "username_claimed": "naveennamani877"
  },
  "HackerNews": {
    "__comment__": "First errMsg invalid, second errMsg rate limited. Not ideal. Adjust for better rate limit filtering.",
    "errorMsg": [
      "No such user.",
      "Sorry."
    ],
    "errorType": "message",
    "url": "https://news.ycombinator.com/user?id={}",
    "urlMain": "https://news.ycombinator.com/",
    "username_claimed": "blue"
  },
  "HackerOne": {
    "errorMsg": "Page not found",
    "errorType": "message",
    "url": "https://hackerone.com/{}",
    "urlMain": "https://hackerone.com/",
    "username_claimed": "stok"
  },
  "HackerRank": {
    "errorMsg": "Something went wrong",
    "errorType": "message",
    "regexCheck": "^[^.]*?$",
    "url": "https://hackerrank.com/{}",
    "urlMain": "https://hackerrank.com/",
    "username_claimed": "satznova"
  },
  "Harvard Scholar": {
    "errorType": "status_code",
    "url": "https://scholar.harvard.edu/{}",
    "urlMain": "https://scholar.harvard.edu/",
    "username_claimed": "ousmanekane"
  },
  "Hashnode": {
    "errorType": "status_code",
    "url": "https://hashnode.com/@{}",
    "urlMain": "https://hashnode.com",
    "username_claimed": "blue"
  },
  "Heavy-R": {
    "errorMsg": "Channel not found",
    "errorType": "message",
    "isNSFW": true,
    "url": "https://www.heavy-r.com/user/{}",
    "urlMain": "https://www.heavy-r.com/",
    "username_claimed": "kilroy222"
  },
  "Holopin": {
    "errorMsg": "true",
    "errorType": "message",
    "request_method": "POST",
    "request_payload": {
      "username": "{}"
    },
    "url": "https://holopin.io/@{}",
    "urlMain": "https://holopin.io",
    "urlProbe": "https://www.holopin.io/api/auth/username",
    "username_claimed": "red"
  },
  "Houzz": {
    "errorMsg": "The page you requested was not found.",
    "errorType": "message",
    "url": "https://houzz.com/user/{}",
    "urlMain": "https://houzz.com/",
    "username_claimed": "blue"
  },
  "HubPages": {
    "errorType": "status_code",
    "url": "https://hubpages.com/@{}",
    "urlMain": "https://hubpages.com/",
    "username_claimed": "blue"
  },
  "Hubski": {
    "errorMsg": "No such user",
    "errorType": "message",
    "url": "https://hubski.com/user/{}",
    "urlMain": "https://hubski.com/",
    "username_claimed": "blue"
  },
  "HudsonRock": {
    "errorMsg": "No results",
    "errorType": "message",
    "url": "https://cavalier.hudsonrock.com/api/json/v2/osint-tools/search-by-username?username={}",
    "urlMain": "https://hudsonrock.com",
    "username_claimed": "testadmin"
  },
  "ICQ": {
    "errorType": "status_code",
    "url": "https://icq.im/{}/en",
    "urlMain": "https://icq.com/",
    "username_claimed": "Micheal"
  },
  "IFTTT": {
    "errorType": "status_code",
    "regexCheck": "^[A-Za-z0-9]{3,35}$",
    "url": "https://www.ifttt.com/p/{}",
    "urlMain": "https://www.ifttt.com/",
    "username_claimed": "blue"
  },
  "IRC-Galleria": {
    "errorType": "response_url",
    "errorUrl": "https://irc-galleria.net/users/search?username={}",
    "url": "https://irc-galleria.net/user/{}",
    "urlMain": "https://irc-galleria.net/",
    "username_claimed": "appas"
  },
  "Icons8 Community": {
    "errorType": "status_code",
    "url": "https://community.icons8.com/u/{}/summary",
    "urlMain": "https://community.icons8.com/",
    "username_claimed": "thefourCraft"
  },
  "Image Fap": {
    "errorMsg": "Not found",
    "errorType": "message",
    "isNSFW": true,
    "url": "https://www.imagefap.com/profile/{}",
    "urlMain": "https://www.imagefap.com/",
    "username_claimed": "blue"
  },
  "ImgUp.cz": {
    "errorType": "status_code",
    "url": "https://imgup.cz/{}",
    "urlMain": "https://imgup.cz/",
    "username_claimed": "adam"
  },
  "Imgur": {
    "errorType": "status_code",
    "url": "https://imgur.com/user/{}",
    "urlMain": "https://imgur.com/",
    "urlProbe": "https://api.imgur.com/account/v1/accounts/{}?client_id=546c25a59c58ad7",
    "username_claimed": "blue"
  },
  "Instagram": {
    "errorType": "status_code",
    "url": "https://instagram.com/{}",
    "urlMain": "https://instagram.com/",
    "urlProbe": "https://www.picuki.com/profile/{}",
    "username_claimed": "instagram"
  },
  "Instructables": {
    "errorType": "status_code",
    "url": "https://www.instructables.com/member/{}",
    "urlMain": "https://www.instructables.com/",
    "urlProbe": "https://www.instructables.com/json-api/showAuthorExists?screenName={}",
    "username_claimed": "blue"
  },
  "Intigriti": {
    "errorType": "status_code",
    "regexCheck": "[a-z0-9_]{1,25}",
    "request_method": "GET",
    "url": "https://app.intigriti.com/profile/{}",
    "urlMain": "https://app.intigriti.com",
    "urlProbe": "https://api.intigriti.com/user/public/profile/{}",
    "username_claimed": "blue"
  },
  "Ionic Forum": {
    "errorType": "status_code",
    "url": "https://forum.ionicframework.com/u/{}",
    "urlMain": "https://forum.ionicframework.com/",
    "username_claimed": "theblue222"
  },
  "Issuu": {
    "errorType": "status_code",
    "url": "https://issuu.com/{}",
    "urlMain": "https://issuu.com/",
    "username_claimed": "jenny"
  },
  "Itch.io": {
    "errorType": "status_code",
    "regexCheck": "^[a-zA-Z0-9@_-]$",
    "url": "https://{}.itch.io/",
    "urlMain": "https://itch.io/",
    "username_claimed": "blue"
  },
  "Itemfix": {
    "errorMsg": "<title>ItemFix - Channel: </title>",
    "errorType": "message",
    "url": "https://www.itemfix.com/c/{}",
    "urlMain": "https://www.itemfix.com/",
    "username_claimed": "blue"
  },
  "Jellyfin Weblate": {
    "errorType": "status_code",
    "regexCheck": "^[a-zA-Z0-9@._-]{1,150}$",
    "url": "https://translate.jellyfin.org/user/{}/",
    "urlMain": "https://translate.jellyfin.org/",
    "username_claimed": "EraYaN"
  },
  "Jimdo": {
    "errorType": "status_code",
    "regexCheck": "^[a-zA-Z0-9@_-]$",
    "url": "https://{}.jimdosite.com",
    "urlMain": "https://jimdosite.com/",
    "username_claimed": "jenny"
  },
  "Joplin Forum": {
    "errorType": "status_code",
    "url": "https://discourse.joplinapp.org/u/{}",
    "urlMain": "https://discourse.joplinapp.org/",
    "username_claimed": "laurent"
  },
  "KEAKR": {
    "errorType": "status_code",
    "url": "https://www.keakr.com/en/profile/{}",
    "urlMain": "https://www.keakr.com/",
    "username_claimed": "beats"
  },
  "Kaggle": {
    "errorType": "status_code",
    "url": "https://www.kaggle.com/{}",
    "urlMain": "https://www.kaggle.com/",
    "username_claimed": "dansbecker"
  },
  "Keybase": {
    "errorType": "status_code",
    "url": "https://keybase.io/{}",
    "urlMain": "https://keybase.io/",
    "username_claimed": "blue"
  },
  "Kick": {
    "__comment__": "Cloudflare. Only viable when proxied.",
    "errorMsg": "Not Found",
    "errorType": "message",
    "url": "https://kick.com/{}",
    "urlMain": "https://kick.com/",
    "urlProbe": "https://kick.com/api/v2/channels/{}",
    "username_claimed": "blue"
  },
  "Kik": {
    "errorMsg": "The page you requested was not found",
    "errorType": "message",
    "url": "https://kik.me/{}",
    "urlMain": "http://kik.me/",
    "urlProbe": "https://ws2.kik.com/user/{}",
    "username_claimed": "blue"
  },
  "Kongregate": {
    "errorType": "status_code",
    "headers": {
      "Accept": "text/html",
      "User-Agent": "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/116.0"
    },
    "regexCheck": "^[a-zA-Z][a-zA-Z0-9_-]*$",
    "url": "https://www.kongregate.com/accounts/{}",
    "urlMain": "https://www.kongregate.com/",
    "username_claimed": "blue"
  },
  "LOR": {
    "errorType": "status_code",
    "url": "https://www.linux.org.ru/people/{}/profile",
    "urlMain": "https://linux.org.ru/",
    "username_claimed": "red"
  },
  "Launchpad": {
    "errorType": "status_code",
    "url": "https://launchpad.net/~{}",
    "urlMain": "https://launchpad.net/",
    "username_claimed": "blue"
  },
  "LeetCode": {
    "errorType": "status_code",
    "url": "https://leetcode.com/{}",
    "urlMain": "https://leetcode.com/",
    "username_claimed": "blue"
  },
  "LessWrong": {
    "errorType": "status_code",
    "url": "https://www.lesswrong.com/users/@{}",
    "urlMain": "https://www.lesswrong.com/",
    "username_claimed": "blue"
  },
  "Letterboxd": {
    "errorMsg": "Sorry, we can\u2019t find the page you\u2019ve requested.",
    "errorType": "message",
    "url": "https://letterboxd.com/{}",
    "urlMain": "https://letterboxd.com/",
    "username_claimed": "blue"
  },
  "LibraryThing": {
    "errorMsg": "Catalog your books online",
    "errorType": "message",
    "url": "https://www.librarything.com/profile/{}",
    "urlMain": "https://www.librarything.com/",
    "username_claimed": "blue"
  },
  "Lichess": {
    "errorMsg": "Page not found!",
    "errorType": "message",
    "url": "https://lichess.org/@/{}",
    "urlMain": "https://lichess.org",
    "username_claimed": "blue"
  },
  "LinkedIn": {
    "errorType": "status_code",
    "headers": {
      "User-Agent": "Mozilla/5.0 AppleWebKit/537.36 (KHTML, like Gecko; compatible; Googlebot/2.1; +http://www.google.com/bot.html) Chrome/W.X.Y.Z Safari/537.36"
    },
    "regexCheck": "^[a-zA-Z0-9]{3,100}$",
    "request_method": "GET",
    "url": "https://linkedin.com/in/{}",
    "urlMain": "https://linkedin.com",
    "username_claimed": "paulpfeister"
  },
  "Linktree": {
    "errorMsg": "\"statusCode\":404",
    "errorType": "message",
    "regexCheck": "^[\\w\\.]{2,30}$",
    "url": "https://linktr.ee/{}",
    "urlMain": "https://linktr.ee/",
    "username_claimed": "anne"
  },
  "Listed": {
    "errorType": "response_url",
    "errorUrl": "https://listed.to/@{}",
    "url": "https://listed.to/@{}",
    "urlMain": "https://listed.to/",
    "username_claimed": "listed"
  },
  "LiveJournal": {
    "errorType": "status_code",
    "regexCheck": "^[a-zA-Z][a-zA-Z0-9_-]*$",
    "url": "https://{}.livejournal.com",
    "urlMain": "https://www.livejournal.com/",
    "username_claimed": "blue"
  },
  "Lobsters": {
    "errorType": "status_code",
    "regexCheck": "[A-Za-z0-9][A-Za-z0-9_-]{0,24}",
    "url": "https://lobste.rs/u/{}",
    "urlMain": "https://lobste.rs/",
    "username_claimed": "jcs"
  },
  "LottieFiles": {
    "errorType": "status_code",
    "url": "https://lottiefiles.com/{}",
    "urlMain": "https://lottiefiles.com/",
    "username_claimed": "lottiefiles"
  },
  "LushStories": {
    "errorType": "status_code",
    "isNSFW": true,
    "url": "https://www.lushstories.com/profile/{}",
    "urlMain": "https://www.lushstories.com/",
    "username_claimed": "chris_brown"
  },
  "MMORPG Forum": {
    "errorType": "status_code",
    "url": "https://forums.mmorpg.com/profile/{}",
    "urlMain": "https://forums.mmorpg.com/",
    "username_claimed": "goku"
  },
  "Mapify": {
    "errorType": "response_url",
    "errorUrl": "https://mapify.travel/{}",
    "url": "https://mapify.travel/{}",
    "urlMain": "https://mapify.travel/",
    "username_claimed": "mapify"
  },
  "Medium": {
    "errorMsg": "<body",
    "errorType": "message",
    "url": "https://medium.com/@{}",
    "urlMain": "https://medium.com/",
    "urlProbe": "https://medium.com/feed/@{}",
    "username_claimed": "blue"
  },
  "Memrise": {
    "errorType": "status_code",
    "url": "https://www.memrise.com/user/{}/",
    "urlMain": "https://www.memrise.com/",
    "username_claimed": "blue"
  },
  "Minecraft": {
    "errorCode": 204,
    "errorType": "status_code",
    "url": "https://api.mojang.com/users/profiles/minecraft/{}",
    "urlMain": "https://minecraft.net/",
    "username_claimed": "blue"
  },
  "MixCloud": {
    "errorType": "status_code",
    "url": "https://www.mixcloud.com/{}/",
    "urlMain": "https://www.mixcloud.com/",
    "urlProbe": "https://api.mixcloud.com/{}/",
    "username_claimed": "jenny"
  },
  "Monkeytype": {
    "errorType": "status_code",
    "url": "https://monkeytype.com/profile/{}",
    "urlMain": "https://monkeytype.com/",
    "urlProbe": "https://api.monkeytype.com/users/{}/profile",
    "username_claimed": "Lost_Arrow"
  },
  "Motherless": {
    "errorMsg": "no longer a member",
    "errorType": "message",
    "isNSFW": true,
    "url": "https://motherless.com/m/{}",
    "urlMain": "https://motherless.com/",
    "username_claimed": "hacker"
  },
  "Motorradfrage": {
    "errorType": "status_code",
    "url": "https://www.motorradfrage.net/nutzer/{}",
    "urlMain": "https://www.motorradfrage.net/",
    "username_claimed": "gutefrage"
  },
  "MyAnimeList": {
    "errorType": "status_code",
    "url": "https://myanimelist.net/profile/{}",
    "urlMain": "https://myanimelist.net/",
    "username_claimed": "blue"
  },
  "MyMiniFactory": {
    "errorType": "status_code",
    "url": "https://www.myminifactory.com/users/{}",
    "urlMain": "https://www.myminifactory.com/",
    "username_claimed": "blue"
  },
  "Mydramalist": {
    "errorMsg": "Sign in - MyDramaList",
    "errorType": "message",
    "url": "https://www.mydramalist.com/profile/{}",
    "urlMain": "https://mydramalist.com",
    "username_claimed": "elhadidy12398"
  },
  "Myspace": {
    "errorType": "status_code",
    "url": "https://myspace.com/{}",
    "urlMain": "https://myspace.com/",
    "username_claimed": "blue"
  },
  "NICommunityForum": {
    "errorMsg": "The page you were looking for could not be found.",
    "errorType": "message",
    "url": "https://community.native-instruments.com/profile/{}",
    "urlMain": "https://www.native-instruments.com/forum/",
    "username_claimed": "jambert"
  },
  "NationStates Nation": {
    "errorMsg": "Was this your nation? It may have ceased to exist due to inactivity, but can rise again!",
    "errorType": "message",
    "url": "https://nationstates.net/nation={}",
    "urlMain": "https://nationstates.net",
    "username_claimed": "the_holy_principality_of_saint_mark"
  },
  "NationStates Region": {
    "errorMsg": "does not exist.",
    "errorType": "message",
    "url": "https://nationstates.net/region={}",
    "urlMain": "https://nationstates.net",
    "username_claimed": "the_west_pacific"
  },
  "Naver": {
    "errorType": "status_code",
    "url": "https://blog.naver.com/{}",
    "urlMain": "https://naver.com",
    "username_claimed": "blue"
  },
  "Needrom": {
    "errorType": "status_code",
    "url": "https://www.needrom.com/author/{}/",
    "urlMain": "https://www.needrom.com/",
    "username_claimed": "needrom"
  },
  "Newgrounds": {
    "errorType": "status_code",
    "regexCheck": "^[a-zA-Z][a-zA-Z0-9_-]*$",
    "url": "https://{}.newgrounds.com",
    "urlMain": "https://newgrounds.com",
    "username_claimed": "blue"
  },
  "Nextcloud Forum": {
    "errorType": "status_code",
    "regexCheck": "^(?![.-])[a-zA-Z0-9_.-]{3,20}$",
    "url": "https://help.nextcloud.com/u/{}/summary",
    "urlMain": "https://nextcloud.com/",
    "username_claimed": "blue"
  },
  "Nightbot": {
    "errorType": "status_code",
    "url": "https://nightbot.tv/t/{}/commands",
    "urlMain": "https://nightbot.tv/",
    "urlProbe": "https://api.nightbot.tv/1/channels/t/{}",
    "username_claimed": "green"
  },
  "Ninja Kiwi": {
    "errorType": "response_url",
    "errorUrl": "https://ninjakiwi.com/profile/{}",
    "url": "https://ninjakiwi.com/profile/{}",
    "urlMain": "https://ninjakiwi.com/",
    "username_claimed": "Kyruko"
  },
  "NintendoLife": {
    "errorType": "status_code",
    "url": "https://www.nintendolife.com/users/{}",
    "urlMain": "https://www.nintendolife.com/",
    "username_claimed": "goku"
  },
  "NitroType": {
    "errorMsg": "<title>Nitro Type | Competitive Typing Game | Race Your Friends</title>",
    "errorType": "message",
    "url": "https://www.nitrotype.com/racer/{}",
    "urlMain": "https://www.nitrotype.com/",
    "username_claimed": "jianclash"
  },
  "NotABug.org": {
    "errorType": "status_code",
    "url": "https://notabug.org/{}",
    "urlMain": "https://notabug.org/",
    "urlProbe": "https://notabug.org/{}/followers",
    "username_claimed": "red"
  },
  "Nyaa.si": {
    "errorType": "status_code",
    "url": "https://nyaa.si/user/{}",
    "urlMain": "https://nyaa.si/",
    "username_claimed": "blue"
  },
  "OGUsers": {
    "errorType": "status_code",
    "url": "https://ogu.gg/{}",
    "urlMain": "https://ogu.gg/",
    "username_claimed": "ogusers"
  },
  "OpenStreetMap": {
    "errorType": "status_code",
    "regexCheck": "^[^.]*?$",
    "url": "https://www.openstreetmap.org/user/{}",
    "urlMain": "https://www.openstreetmap.org/",
    "username_claimed": "blue"
  },
  "Opensource": {
    "errorType": "status_code",
    "url": "https://opensource.com/users/{}",
    "urlMain": "https://opensource.com/",
    "username_claimed": "red"
  },
  "OurDJTalk": {
    "errorMsg": "The specified member cannot be found",
    "errorType": "message",
    "url": "https://ourdjtalk.com/members?username={}",
    "urlMain": "https://ourdjtalk.com/",
    "username_claimed": "steve"
  },
  "PCGamer": {
    "errorMsg": "The specified member cannot be found. Please enter a member's entire name.",
    "errorType": "message",
    "url": "https://forums.pcgamer.com/members/?username={}",
    "urlMain": "https://pcgamer.com",
    "username_claimed": "admin"
  },
  "PSNProfiles.com": {
    "errorType": "response_url",
    "errorUrl": "https://psnprofiles.com/?psnId={}",
    "url": "https://psnprofiles.com/{}",
    "urlMain": "https://psnprofiles.com/",
    "username_claimed": "blue"
  },
  "Packagist": {
    "errorType": "response_url",
    "errorUrl": "https://packagist.org/search/?q={}&reason=vendor_not_found",
    "url": "https://packagist.org/packages/{}/",
    "urlMain": "https://packagist.org/",
    "username_claimed": "psr"
  },
  "Pastebin": {
    "errorMsg": "Not Found (#404)",
    "errorType": "message",
    "url": "https://pastebin.com/u/{}",
    "urlMain": "https://pastebin.com/",
    "username_claimed": "blue"
  },
  "Patreon": {
    "errorType": "status_code",
    "url": "https://www.patreon.com/{}",
    "urlMain": "https://www.patreon.com/",
    "username_claimed": "blue"
  },
  "PentesterLab": {
    "errorType": "status_code",
    "regexCheck": "^[\\w]{4,30}$",
    "url": "https://pentesterlab.com/profile/{}",
    "urlMain": "https://pentesterlab.com/",
    "username_claimed": "0day"
  },
  "PepperIT": {
    "errorMsg": "La pagina che hai provato a raggiungere non si trova qui",
    "errorType": "message",
    "url": "https://www.pepper.it/profile/{}/overview",
    "urlMain": "https://www.pepper.it",
    "username_claimed": "asoluinostrisca"
  },
  "Periscope": {
    "errorType": "status_code",
    "url": "https://www.periscope.tv/{}/",
    "urlMain": "https://www.periscope.tv/",
    "username_claimed": "blue"
  },
  "Pinkbike": {
    "errorType": "status_code",
    "regexCheck": "^[^.]*?$",
    "url": "https://www.pinkbike.com/u/{}/",
    "urlMain": "https://www.pinkbike.com/",
    "username_claimed": "blue"
  },
  "PlayStore": {
    "errorType": "status_code",
    "url": "https://play.google.com/store/apps/developer?id={}",
    "urlMain": "https://play.google.com/store",
    "username_claimed": "Facebook"
  },
  "PocketStars": {
    "errorMsg": "Join Your Favorite Adult Stars",
    "errorType": "message",
    "isNSFW": true,
    "url": "https://pocketstars.com/{}",
    "urlMain": "https://pocketstars.com/",
    "username_claimed": "hacker"
  },
  "Pokemon Showdown": {
    "errorType": "status_code",
    "url": "https://pokemonshowdown.com/users/{}",
    "urlMain": "https://pokemonshowdown.com",
    "username_claimed": "blue"
  },
  "Polarsteps": {
    "errorType": "status_code",
    "url": "https://polarsteps.com/{}",
    "urlMain": "https://polarsteps.com/",
    "urlProbe": "https://api.polarsteps.com/users/byusername/{}",
    "username_claimed": "james"
  },
  "Polygon": {
    "errorType": "status_code",
    "url": "https://www.polygon.com/users/{}",
    "urlMain": "https://www.polygon.com/",
    "username_claimed": "swiftstickler"
  },
  "Polymart": {
    "errorType": "response_url",
    "errorUrl": "https://polymart.org/user/-1",
    "url": "https://polymart.org/user/{}",
    "urlMain": "https://polymart.org/",
    "username_claimed": "craciu25yt"
  },
  "Pornhub": {
    "errorType": "status_code",
    "isNSFW": true,
    "url": "https://pornhub.com/users/{}",
    "urlMain": "https://pornhub.com/",
    "username_claimed": "blue"
  },
  "ProductHunt": {
    "errorMsg": "We seem to have lost this page",
    "errorType": "message",
    "url": "https://www.producthunt.com/@{}",
    "urlMain": "https://www.producthunt.com/",
    "username_claimed": "jenny"
  },
  "PromoDJ": {
    "errorType": "status_code",
    "url": "http://promodj.com/{}",
    "urlMain": "http://promodj.com/",
    "username_claimed": "blue"
  },
  "PyPi": {
    "errorType": "status_code",
    "url": "https://pypi.org/user/{}",
    "urlMain": "https://pypi.org",
    "username_claimed": "Blue"
  },
  "Rajce.net": {
    "errorType": "status_code",
    "regexCheck": "^[a-zA-Z0-9@_-]$",
    "url": "https://{}.rajce.idnes.cz/",
    "urlMain": "https://www.rajce.idnes.cz/",
    "username_claimed": "blue"
  },
  "Rate Your Music": {
    "errorType": "status_code",
    "url": "https://rateyourmusic.com/~{}",
    "urlMain": "https://rateyourmusic.com/",
    "username_claimed": "blue"
  },
  "Rclone Forum": {
    "errorType": "status_code",
    "url": "https://forum.rclone.org/u/{}",
    "urlMain": "https://forum.rclone.org/",
    "username_claimed": "ncw"
  },
  "RedTube": {
    "errorType": "status_code",
    "isNSFW": true,
    "url": "https://www.redtube.com/users/{}",
    "urlMain": "https://www.redtube.com/",
    "username_claimed": "hacker"
  },
  "Redbubble": {
    "errorType": "status_code",
    "url": "https://www.redbubble.com/people/{}",
    "urlMain": "https://www.redbubble.com/",
    "username_claimed": "blue"
  },
  "Reddit": {
    "errorMsg": "Sorry, nobody on Reddit goes by that name.",
    "errorType": "message",
    "headers": {
      "accept-language": "en-US,en;q=0.9"
    },
    "url": "https://www.reddit.com/user/{}",
    "urlMain": "https://www.reddit.com/",
    "username_claimed": "blue"
  },
  "Reisefrage": {
    "errorType": "status_code",
    "url": "https://www.reisefrage.net/nutzer/{}",
    "urlMain": "https://www.reisefrage.net/",
    "username_claimed": "reisefrage"
  },
  "Replit.com": {
    "errorType": "status_code",
    "url": "https://replit.com/@{}",
    "urlMain": "https://replit.com/",
    "username_claimed": "blue"
  },
  "ResearchGate": {
    "errorType": "response_url",
    "errorUrl": "https://www.researchgate.net/directory/profiles",
    "regexCheck": "\\w+_\\w+",
    "url": "https://www.researchgate.net/profile/{}",
    "urlMain": "https://www.researchgate.net/",
    "username_claimed": "John_Smith"
  },
  "ReverbNation": {
    "errorMsg": "Sorry, we couldn't find that page",
    "errorType": "message",
    "url": "https://www.reverbnation.com/{}",
    "urlMain": "https://www.reverbnation.com/",
    "username_claimed": "blue"
  },
  "Roblox": {
    "errorMsg": "Page cannot be found or no longer exists",
    "errorType": "message",
    "url": "https://www.roblox.com/user.aspx?username={}",
    "urlMain": "https://www.roblox.com/",
    "username_claimed": "bluewolfekiller"
  },
  "RocketTube": {
    "errorMsg": "OOPS! Houston, we have a problem",
    "errorType": "message",
    "isNSFW": true,
    "url": "https://www.rockettube.com/{}",
    "urlMain": "https://www.rockettube.com/",
    "username_claimed": "Tatteddick5600"
  },
  "RoyalCams": {
    "errorType": "status_code",
    "url": "https://royalcams.com/profile/{}",
    "urlMain": "https://royalcams.com",
    "username_claimed": "asuna-black"
  },
  "RubyGems": {
    "errorType": "status_code",
    "regexCheck": "^[a-zA-Z][a-zA-Z0-9_-]{1,40}",
    "url": "https://rubygems.org/profiles/{}",
    "urlMain": "https://rubygems.org/",
    "username_claimed": "blue"
  },
  "Rumble": {
    "errorType": "status_code",
    "url": "https://rumble.com/user/{}",
    "urlMain": "https://rumble.com/",
    "username_claimed": "John"
  },
  "RuneScape": {
    "errorMsg": "{\"error\":\"NO_PROFILE\",\"loggedIn\":\"false\"}",
    "errorType": "message",
    "regexCheck": "^(?! )[\\w -]{1,12}(?<! )$",
    "url": "https://apps.runescape.com/runemetrics/app/overview/player/{}",
    "urlMain": "https://www.runescape.com/",
    "urlProbe": "https://apps.runescape.com/runemetrics/profile/profile?user={}",
    "username_claimed": "L33"
  },
  "SWAPD": {
    "errorType": "status_code",
    "url": "https://swapd.co/u/{}",
    "urlMain": "https://swapd.co/",
    "username_claimed": "swapd"
  },
  "Sbazar.cz": {
    "errorType": "status_code",
    "url": "https://www.sbazar.cz/{}",
    "urlMain": "https://www.sbazar.cz/",
    "username_claimed": "blue"
  },
  "Scratch": {
    "errorType": "status_code",
    "url": "https://scratch.mit.edu/users/{}",
    "urlMain": "https://scratch.mit.edu/",
    "username_claimed": "griffpatch"
  },
  "Scribd": {
    "errorMsg": "Page not found",
    "errorType": "message",
    "url": "https://www.scribd.com/{}",
    "urlMain": "https://www.scribd.com/",
    "username_claimed": "blue"
  },
  "ShitpostBot5000": {
    "errorType": "status_code",
    "url": "https://www.shitpostbot.com/user/{}",
    "urlMain": "https://www.shitpostbot.com/",
    "username_claimed": "blue"
  },
  "Shpock": {
    "errorType": "status_code",
    "url": "https://www.shpock.com/shop/{}/items",
    "urlMain": "https://www.shpock.com/",
    "username_claimed": "user"
  },
  "Signal": {
    "errorMsg": "Oops! That page doesn\u2019t exist or is private.",
    "errorType": "message",
    "url": "https://community.signalusers.org/u/{}",
    "urlMain": "https://community.signalusers.org",
    "username_claimed": "jlund"
  },
  "Sketchfab": {
    "errorType": "status_code",
    "url": "https://sketchfab.com/{}",
    "urlMain": "https://sketchfab.com/",
    "username_claimed": "blue"
  },
  "Slack": {
    "errorType": "status_code",
    "regexCheck": "^[a-zA-Z][a-zA-Z0-9_-]*$",
    "url": "https://{}.slack.com",
    "urlMain": "https://slack.com",
    "username_claimed": "blue"
  },
  "Slant": {
    "errorType": "status_code",
    "regexCheck": "^.{2,32}$",
    "url": "https://www.slant.co/users/{}",
    "urlMain": "https://www.slant.co/",
    "username_claimed": "blue"
  },
  "Slashdot": {
    "errorMsg": "user you requested does not exist",
    "errorType": "message",
    "url": "https://slashdot.org/~{}",
    "urlMain": "https://slashdot.org",
    "username_claimed": "blue"
  },
  "SlideShare": {
    "errorType": "status_code",
    "url": "https://slideshare.net/{}",
    "urlMain": "https://slideshare.net/",
    "username_claimed": "blue"
  },
  "Slides": {
    "errorCode": 204,
    "errorType": "status_code",
    "url": "https://slides.com/{}",
    "urlMain": "https://slides.com/",
    "username_claimed": "blue"
  },
  "SmugMug": {
    "errorType": "status_code",
    "url": "https://{}.smugmug.com",
    "urlMain": "https://smugmug.com",
    "username_claimed": "winchester"
  },
  "Smule": {
    "errorMsg": "Smule | Page Not Found (404)",
    "errorType": "message",
    "url": "https://www.smule.com/{}",
    "urlMain": "https://www.smule.com/",
    "username_claimed": "blue"
  },
  "Snapchat": {
    "errorType": "status_code",
    "regexCheck": "^[a-z][a-z-_.]{3,15}",
    "request_method": "GET",
    "url": "https://www.snapchat.com/add/{}",
    "urlMain": "https://www.snapchat.com",
    "username_claimed": "teamsnapchat"
  },
  "SoundCloud": {
    "errorType": "status_code",
    "url": "https://soundcloud.com/{}",
    "urlMain": "https://soundcloud.com/",
    "username_claimed": "blue"
  },
  "SourceForge": {
    "errorType": "status_code",
    "url": "https://sourceforge.net/u/{}",
    "urlMain": "https://sourceforge.net/",
    "username_claimed": "blue"
  },
  "SoylentNews": {
    "errorMsg": "The user you requested does not exist, no matter how much you wish this might be the case.",
    "errorType": "message",
    "url": "https://soylentnews.org/~{}",
    "urlMain": "https://soylentnews.org",
    "username_claimed": "adam"
  },
  "Speedrun.com": {
    "errorMsg": "Not found",
    "errorType": "message",
    "url": "https://speedrun.com/user/{}",
    "urlMain": "https://speedrun.com/",
    "username_claimed": "3Tau"
  },
  "Spells8": {
    "errorType": "status_code",
    "url": "https://forum.spells8.com/u/{}",
    "urlMain": "https://spells8.com",
    "username_claimed": "susurrus"
  },
  "Splice": {
    "errorType": "status_code",
    "url": "https://splice.com/{}",
    "urlMain": "https://splice.com/",
    "username_claimed": "splice"
  },
  "Splits.io": {
    "errorType": "status_code",
    "regexCheck": "^[^.]*?$",
    "url": "https://splits.io/users/{}",
    "urlMain": "https://splits.io",
    "username_claimed": "cambosteve"
  },
  "Sporcle": {
    "errorType": "status_code",
    "url": "https://www.sporcle.com/user/{}/people",
    "urlMain": "https://www.sporcle.com/",
    "username_claimed": "blue"
  },
  "Sportlerfrage": {
    "errorType": "status_code",
    "url": "https://www.sportlerfrage.net/nutzer/{}",
    "urlMain": "https://www.sportlerfrage.net/",
    "username_claimed": "sportlerfrage"
  },
  "SportsRU": {
    "errorType": "status_code",
    "url": "https://www.sports.ru/profile/{}/",
    "urlMain": "https://www.sports.ru/",
    "username_claimed": "blue"
  },
  "Spotify": {
    "errorType": "status_code",
    "headers": {
      "user-agent": "PostmanRuntime/7.29.2"
    },
    "url": "https://open.spotify.com/user/{}",
    "urlMain": "https://open.spotify.com/",
    "username_claimed": "blue"
  },
  "Star Citizen": {
    "errorMsg": "404",
    "errorType": "message",
    "url": "https://robertsspaceindustries.com/citizens/{}",
    "urlMain": "https://robertsspaceindustries.com/",
    "username_claimed": "blue"
  },
  "Steam Community (Group)": {
    "errorMsg": "No group could be retrieved for the given URL",
    "errorType": "message",
    "url": "https://steamcommunity.com/groups/{}",
    "urlMain": "https://steamcommunity.com/",
    "username_claimed": "blue"
  },
  "Steam Community (User)": {
    "errorMsg": "The specified profile could not be found",
    "errorType": "message",
    "url": "https://steamcommunity.com/id/{}/",
    "urlMain": "https://steamcommunity.com/",
    "username_claimed": "blue"
  },
  "Strava": {
    "errorMsg": "Strava | Running, Cycling &amp; Hiking App - Train, Track &amp; Share",
    "errorType": "message",
    "regexCheck": "^[^.]*?$",
    "url": "https://www.strava.com/athletes/{}",
    "urlMain": "https://www.strava.com/",
    "username_claimed": "blue"
  },
  "SublimeForum": {
    "errorType": "status_code",
    "url": "https://forum.sublimetext.com/u/{}",
    "urlMain": "https://forum.sublimetext.com/",
    "username_claimed": "blue"
  },
  "TETR.IO": {
    "errorMsg": "No such user!",
    "errorType": "message",
    "url": "https://ch.tetr.io/u/{}",
    "urlMain": "https://tetr.io",
    "urlProbe": "https://ch.tetr.io/api/users/{}",
    "username_claimed": "osk"
  },
  "TLDR Legal": {
    "errorType": "status_code",
    "regexCheck": "^[a-zA-Z0-9]{3,20}$",
    "url": "https://tldrlegal.com/users/{}/",
    "urlMain": "https://tldrlegal.com/",
    "username_claimed": "kevin"
  },
  "TRAKTRAIN": {
    "errorType": "status_code",
    "url": "https://traktrain.com/{}",
    "urlMain": "https://traktrain.com/",
    "username_claimed": "traktrain"
  },
  "Telegram": {
    "errorMsg": [
      "<title>Telegram Messenger</title>",
      "If you have <strong>Telegram</strong>, you can contact <a class=\"tgme_username_link\" href=\"tg://resolve?domain="
    ],
    "errorType": "message",
    "regexCheck": "^[a-zA-Z0-9_]{3,32}[^_]$",
    "url": "https://t.me/{}",
    "urlMain": "https://t.me/",
    "username_claimed": "blue"
  },
  "Tellonym.me": {
    "errorType": "status_code",
    "url": "https://tellonym.me/{}",
    "urlMain": "https://tellonym.me/",
    "username_claimed": "blue"
  },
  "Tenor": {
    "errorType": "status_code",
    "regexCheck": "^[A-Za-z0-9_]{2,32}$",
    "url": "https://tenor.com/users/{}",
    "urlMain": "https://tenor.com/",
    "username_claimed": "red"
  },
  "ThemeForest": {
    "errorType": "status_code",
    "url": "https://themeforest.net/user/{}",
    "urlMain": "https://themeforest.net/",
    "username_claimed": "user"
  },
  "TnAFlix": {
    "errorType": "status_code",
    "isNSFW": true,
    "url": "https://www.tnaflix.com/profile/{}",
    "urlMain": "https://www.tnaflix.com/",
    "username_claimed": "hacker"
  },
  "TorrentGalaxy": {
    "errorMsg": "<title>TGx:Can't show details</title>",
    "errorType": "message",
    "regexCheck": "^[A-Za-z0-9]{3,15}$",
    "url": "https://torrentgalaxy.to/profile/{}",
    "urlMain": "https://torrentgalaxy.to/",
    "username_claimed": "GalaxyRG"
  },
  "TradingView": {
    "errorType": "status_code",
    "request_method": "GET",
    "url": "https://www.tradingview.com/u/{}/",
    "urlMain": "https://www.tradingview.com/",
    "username_claimed": "blue"
  },
  "Trakt": {
    "errorType": "status_code",
    "regexCheck": "^[^.]*$",
    "url": "https://www.trakt.tv/users/{}",
    "urlMain": "https://www.trakt.tv/",
    "username_claimed": "blue"
  },
  "TrashboxRU": {
    "errorType": "status_code",
    "regexCheck": "^[A-Za-z0-9_-]{3,16}$",
    "url": "https://trashbox.ru/users/{}",
    "urlMain": "https://trashbox.ru/",
    "username_claimed": "blue"
  },
  "Trawelling": {
    "errorType": "status_code",
    "url": "https://traewelling.de/@{}",
    "urlMain": "https://traewelling.de/",
    "username_claimed": "lassestolley"
  },
  "Trello": {
    "errorMsg": "model not found",
    "errorType": "message",
    "url": "https://trello.com/{}",
    "urlMain": "https://trello.com/",
    "urlProbe": "https://trello.com/1/Members/{}",
    "username_claimed": "blue"
  },
  "TryHackMe": {
    "errorMsg": "{\"success\":false}",
    "errorType": "message",
    "regexCheck": "^[a-zA-Z0-9.]{1,16}$",
    "url": "https://tryhackme.com/p/{}",
    "urlMain": "https://tryhackme.com/",
    "urlProbe": "https://tryhackme.com/api/user/exist/{}",
    "username_claimed": "ashu"
  },
  "Tuna": {
    "errorType": "status_code",
    "regexCheck": "^[a-z0-9]{4,40}$",
    "url": "https://tuna.voicemod.net/user/{}",
    "urlMain": "https://tuna.voicemod.net/",
    "username_claimed": "bob"
  },
  "Tweakers": {
    "errorType": "status_code",
    "url": "https://tweakers.net/gallery/{}",
    "urlMain": "https://tweakers.net",
    "username_claimed": "femme"
  },
  "Twitch": {
    "errorType": "status_code",
    "url": "https://www.twitch.tv/{}",
    "urlMain": "https://www.twitch.tv/",
    "urlProbe": "https://m.twitch.tv/{}",
    "username_claimed": "jenny"
  },
  "Twitter": {
    "errorMsg": "<div class=\"error-panel\"><span>User ",
    "errorType": "message",
    "regexCheck": "^[a-zA-Z0-9_]{1,15}$",
    "url": "https://x.com/{}",
    "urlMain": "https://x.com/",
    "urlProbe": "https://nitter.net/{}",
    "username_claimed": "blue"
  },
  "Typeracer": {
    "errorMsg": "Profile Not Found",
    "errorType": "message",
    "url": "https://data.typeracer.com/pit/profile?user={}",
    "urlMain": "https://typeracer.com",
    "username_claimed": "blue"
  },
  "Ultimate-Guitar": {
    "errorType": "status_code",
    "url": "https://ultimate-guitar.com/u/{}",
    "urlMain": "https://ultimate-guitar.com/",
    "username_claimed": "blue"
  },
  "Unsplash": {
    "errorType": "status_code",
    "regexCheck": "^[a-z0-9_]{1,60}$",
    "url": "https://unsplash.com/@{}",
    "urlMain": "https://unsplash.com/",
    "username_claimed": "jenny"
  },
  "Untappd": {
    "errorType": "status_code",
    "url": "https://untappd.com/user/{}",
    "urlMain": "https://untappd.com/",
    "username_claimed": "untappd"
  },
  "VK": {
    "errorType": "response_url",
    "errorUrl": "https://www.quora.com/profile/{}",
    "url": "https://vk.com/{}",
    "urlMain": "https://vk.com/",
    "username_claimed": "brown"
  },
  "VSCO": {
    "errorType": "status_code",
    "url": "https://vsco.co/{}",
    "urlMain": "https://vsco.co/",
    "username_claimed": "blue"
  },
  "Velomania": {
    "errorMsg": "\u041f\u043e\u043b\u044c\u0437\u043e\u0432\u0430\u0442\u0435\u043b\u044c \u043d\u0435 \u0437\u0430\u0440\u0435\u0433\u0438\u0441\u0442\u0440\u0438\u0440\u043e\u0432\u0430\u043d \u0438 \u043d\u0435 \u0438\u043c\u0435\u0435\u0442 \u043f\u0440\u043e\u0444\u0438\u043b\u044f \u0434\u043b\u044f \u043f\u0440\u043e\u0441\u043c\u043e\u0442\u0440\u0430.",
    "errorType": "message",
    "url": "https://forum.velomania.ru/member.php?username={}",
    "urlMain": "https://forum.velomania.ru/",
    "username_claimed": "red"
  },
  "Venmo": {
    "errorMsg": [
      "Venmo | Page Not Found"
    ],
    "errorType": "message",
    "headers": {
      "Host": "account.venmo.com"
    },
    "url": "https://account.venmo.com/u/{}",
    "urlMain": "https://venmo.com/",
    "urlProbe": "https://test1.venmo.com/u/{}",
    "username_claimed": "jenny"
  },
  "Vero": {
    "errorType": "status_code",
    "request_method": "GET",
    "url": "https://vero.co/{}",
    "urlMain": "https://vero.co/",
    "username_claimed": "blue"
  },
  "Vimeo": {
    "errorType": "status_code",
    "url": "https://vimeo.com/{}",
    "urlMain": "https://vimeo.com/",
    "username_claimed": "blue"
  },
  "VirusTotal": {
    "errorType": "status_code",
    "request_method": "GET",
    "url": "https://www.virustotal.com/gui/user/{}",
    "urlMain": "https://www.virustotal.com/",
    "urlProbe": "https://www.virustotal.com/ui/users/{}/avatar",
    "username_claimed": "blue"
  },
  "WICG Forum": {
    "errorType": "status_code",
    "regexCheck": "^(?![.-])[a-zA-Z0-9_.-]{3,20}$",
    "url": "https://discourse.wicg.io/u/{}/summary",
    "urlMain": "https://discourse.wicg.io/",
    "username_claimed": "stefano"
  },
  "Warrior Forum": {
    "errorType": "status_code",
    "url": "https://www.warriorforum.com/members/{}.html",
    "urlMain": "https://www.warriorforum.com/",
    "username_claimed": "blue"
  },
  "Wattpad": {
    "errorType": "status_code",
    "url": "https://www.wattpad.com/user/{}",
    "urlMain": "https://www.wattpad.com/",
    "urlProbe": "https://www.wattpad.com/api/v3/users/{}/",
    "username_claimed": "Dogstho7951"
  },
  "WebNode": {
    "errorType": "status_code",
    "regexCheck": "^[a-zA-Z0-9@_-]$",
    "url": "https://{}.webnode.cz/",
    "urlMain": "https://www.webnode.cz/",
    "username_claimed": "radkabalcarova"
  },
  "Weblate": {
    "errorType": "status_code",
    "regexCheck": "^[a-zA-Z0-9@._-]{1,150}$",
    "url": "https://hosted.weblate.org/user/{}/",
    "urlMain": "https://hosted.weblate.org/",
    "username_claimed": "adam"
  },
  "Weebly": {
    "errorType": "status_code",
    "url": "https://{}.weebly.com/",
    "urlMain": "https://weebly.com/",
    "username_claimed": "blue"
  },
  "Wikidot": {
    "errorMsg": "User does not exist.",
    "errorType": "message",
    "url": "http://www.wikidot.com/user:info/{}",
    "urlMain": "http://www.wikidot.com/",
    "username_claimed": "blue"
  },
  "Wikipedia": {
    "errorMsg": "centralauth-admin-nonexistent:",
    "errorType": "message",
    "url": "https://en.wikipedia.org/wiki/Special:CentralAuth/{}?uselang=qqx",
    "urlMain": "https://www.wikipedia.org/",
    "username_claimed": "Hoadlck"
  },
  "Windy": {
    "errorType": "status_code",
    "url": "https://community.windy.com/user/{}",
    "urlMain": "https://windy.com/",
    "username_claimed": "blue"
  },
  "Wix": {
    "errorType": "status_code",
    "regexCheck": "^[a-zA-Z0-9@_-]$",
    "url": "https://{}.wix.com",
    "urlMain": "https://wix.com/",
    "username_claimed": "support"
  },
  "WolframalphaForum": {
    "errorType": "status_code",
    "url": "https://community.wolfram.com/web/{}/home",
    "urlMain": "https://community.wolfram.com/",
    "username_claimed": "unico"
  },
  "WordPress": {
    "errorType": "response_url",
    "errorUrl": "wordpress.com/typo/?subdomain=",
    "regexCheck": "^[a-zA-Z][a-zA-Z0-9_-]*$",
    "url": "https://{}.wordpress.com/",
    "urlMain": "https://wordpress.com",
    "username_claimed": "blue"
  },
  "WordPressOrg": {
    "errorType": "response_url",
    "errorUrl": "https://wordpress.org",
    "url": "https://profiles.wordpress.org/{}/",
    "urlMain": "https://wordpress.org/",
    "username_claimed": "blue"
  },
  "Wordnik": {
    "errorMsg": "Page Not Found",
    "errorType": "message",
    "regexCheck": "^[a-zA-Z0-9_.+-]{1,40}$",
    "url": "https://www.wordnik.com/users/{}",
    "urlMain": "https://www.wordnik.com/",
    "username_claimed": "blue"
  },
  "Wykop": {
    "errorType": "status_code",
    "url": "https://www.wykop.pl/ludzie/{}",
    "urlMain": "https://www.wykop.pl",
    "username_claimed": "blue"
  },
  "Xbox Gamertag": {
    "errorType": "status_code",
    "url": "https://xboxgamertag.com/search/{}",
    "urlMain": "https://xboxgamertag.com/",
    "username_claimed": "red"
  },
  "Xvideos": {
    "errorType": "status_code",
    "isNSFW": true,
    "url": "https://xvideos.com/profiles/{}",
    "urlMain": "https://xvideos.com/",
    "username_claimed": "blue"
  },
  "YandexMusic": {
    "__comment__": "The first and third errorMsg relate to geo-restrictions and bot detection/captchas.",
    "errorMsg": [
      "\u041e\u0448\u0438\u0431\u043a\u0430 404",
      "<meta name=\"description\" content=\"\u041e\u0442\u043a\u0440\u044b\u0432\u0430\u0439\u0442\u0435 \u043d\u043e\u0432\u0443\u044e \u043c\u0443\u0437\u044b\u043a\u0443 \u043a\u0430\u0436\u0434\u044b\u0439 \u0434\u0435\u043d\u044c.",
      "<input type=\"submit\" class=\"CheckboxCaptcha-Button\""
    ],
    "errorType": "message",
    "url": "https://music.yandex/users/{}/playlists",
    "urlMain": "https://music.yandex",
    "username_claimed": "ya.playlist"
  },
  "YouNow": {
    "errorMsg": "No users found",
    "errorType": "message",
    "url": "https://www.younow.com/{}/",
    "urlMain": "https://www.younow.com/",
    "urlProbe": "https://api.younow.com/php/api/broadcast/info/user={}/",
    "username_claimed": "blue"
  },
  "YouPic": {
    "errorType": "status_code",
    "url": "https://youpic.com/photographer/{}/",
    "urlMain": "https://youpic.com/",
    "username_claimed": "blue"
  },
  "YouPorn": {
    "errorType": "status_code",
    "isNSFW": true,
    "url": "https://youporn.com/uservids/{}",
    "urlMain": "https://youporn.com",
    "username_claimed": "blue"
  },
  "YouTube": {
    "errorType": "status_code",
    "headers": {
      "User-Agent": "Mozilla/5.0 AppleWebKit/537.36 (KHTML, like Gecko; compatible; Googlebot/2.1; +http://www.google.com/bot.html) Chrome/W.X.Y.Z Safari/537.36"
    },
    "url": "https://www.youtube.com/@{}",
    "urlMain": "https://www.youtube.com/",
    "username_claimed": "youtube"
  },
  "akniga": {
    "errorType": "status_code",
    "url": "https://akniga.org/profile/{}",
    "urlMain": "https://akniga.org/profile/blue/",
    "username_claimed": "blue"
  },
  "authorSTREAM": {
    "errorType": "status_code",
    "url": "http://www.authorstream.com/{}/",
    "urlMain": "http://www.authorstream.com/",
    "username_claimed": "blue"
  },
  "babyRU": {
    "errorMsg": "\u0421\u0442\u0440\u0430\u043d\u0438\u0446\u0430, \u043a\u043e\u0442\u043e\u0440\u0443\u044e \u0432\u044b \u0438\u0441\u043a\u0430\u043b\u0438, \u043d\u0435 \u043d\u0430\u0439\u0434\u0435\u043d\u0430",
    "errorType": "message",
    "url": "https://www.baby.ru/u/{}/",
    "urlMain": "https://www.baby.ru/",
    "username_claimed": "blue"
  },
  "babyblogRU": {
    "errorType": "response_url",
    "errorUrl": "https://www.babyblog.ru/",
    "url": "https://www.babyblog.ru/user/{}",
    "urlMain": "https://www.babyblog.ru/",
    "username_claimed": "blue"
  },
  "chaos.social": {
    "errorType": "status_code",
    "url": "https://chaos.social/@{}",
    "urlMain": "https://chaos.social/",
    "username_claimed": "rixx"
  },
  "couchsurfing": {
    "errorType": "status_code",
    "url": "https://www.couchsurfing.com/people/{}",
    "urlMain": "https://www.couchsurfing.com/",
    "username_claimed": "blue"
  },
  "d3RU": {
    "errorType": "status_code",
    "url": "https://d3.ru/user/{}/posts",
    "urlMain": "https://d3.ru/",
    "username_claimed": "blue"
  },
  "dailykos": {
    "errorMsg": "{\"result\":true,\"message\":null}",
    "errorType": "message",
    "url": "https://www.dailykos.com/user/{}",
    "urlMain": "https://www.dailykos.com",
    "urlProbe": "https://www.dailykos.com/signup/check_nickname?nickname={}",
    "username_claimed": "blue"
  },
  "datingRU": {
    "errorType": "status_code",
    "url": "http://dating.ru/{}",
    "urlMain": "http://dating.ru",
    "username_claimed": "blue"
  },
  "devRant": {
    "errorType": "response_url",
    "errorUrl": "https://devrant.com/",
    "url": "https://devrant.com/users/{}",
    "urlMain": "https://devrant.com/",
    "username_claimed": "blue"
  },
  "drive2": {
    "errorType": "status_code",
    "url": "https://www.drive2.ru/users/{}",
    "urlMain": "https://www.drive2.ru/",
    "username_claimed": "blue"
  },
  "eGPU": {
    "errorType": "status_code",
    "url": "https://egpu.io/forums/profile/{}/",
    "urlMain": "https://egpu.io/",
    "username_claimed": "blue"
  },
  "eintracht": {
    "errorType": "status_code",
    "regexCheck": "^[^.]*?$",
    "url": "https://community.eintracht.de/fans/{}",
    "urlMain": "https://eintracht.de",
    "username_claimed": "blue"
  },
  "fixya": {
    "errorType": "status_code",
    "url": "https://www.fixya.com/users/{}",
    "urlMain": "https://www.fixya.com",
    "username_claimed": "adam"
  },
  "fl": {
    "errorType": "status_code",
    "url": "https://www.fl.ru/users/{}",
    "urlMain": "https://www.fl.ru/",
    "username_claimed": "blue"
  },
  "forum_guns": {
    "errorMsg": "action=https://forum.guns.ru/forummisc/blog/search",
    "errorType": "message",
    "url": "https://forum.guns.ru/forummisc/blog/{}",
    "urlMain": "https://forum.guns.ru/",
    "username_claimed": "red"
  },
  "freecodecamp": {
    "errorType": "status_code",
    "url": "https://www.freecodecamp.org/{}",
    "urlMain": "https://www.freecodecamp.org/",
    "urlProbe": "https://api.freecodecamp.org/api/users/get-public-profile?username={}",
    "username_claimed": "naveennamani"
  },
  "furaffinity": {
    "errorMsg": "This user cannot be found.",
    "errorType": "message",
    "url": "https://www.furaffinity.net/user/{}",
    "urlMain": "https://www.furaffinity.net",
    "username_claimed": "jesus"
  },
  "geocaching": {
    "errorType": "status_code",
    "url": "https://www.geocaching.com/p/default.aspx?u={}",
    "urlMain": "https://www.geocaching.com/",
    "username_claimed": "blue"
  },
  "gfycat": {
    "errorType": "status_code",
    "url": "https://gfycat.com/@{}",
    "urlMain": "https://gfycat.com/",
    "username_claimed": "Test"
  },
  "habr": {
    "errorType": "status_code",
    "url": "https://habr.com/ru/users/{}",
    "urlMain": "https://habr.com/",
    "username_claimed": "blue"
  },
  "hackster": {
    "errorType": "status_code",
    "url": "https://www.hackster.io/{}",
    "urlMain": "https://www.hackster.io",
    "username_claimed": "blue"
  },
  "hunting": {
    "errorMsg": "\u0423\u043a\u0430\u0437\u0430\u043d\u043d\u044b\u0439 \u043f\u043e\u043b\u044c\u0437\u043e\u0432\u0430\u0442\u0435\u043b\u044c \u043d\u0435 \u043d\u0430\u0439\u0434\u0435\u043d. \u041f\u043e\u0436\u0430\u043b\u0443\u0439\u0441\u0442\u0430, \u0432\u0432\u0435\u0434\u0438\u0442\u0435 \u0434\u0440\u0443\u0433\u043e\u0435 \u0438\u043c\u044f.",
    "errorType": "message",
    "url": "https://www.hunting.ru/forum/members/?username={}",
    "urlMain": "https://www.hunting.ru/forum/",
    "username_claimed": "red"
  },
  "iMGSRC.RU": {
    "errorType": "response_url",
    "errorUrl": "https://imgsrc.ru/",
    "url": "https://imgsrc.ru/main/user.php?user={}",
    "urlMain": "https://imgsrc.ru/",
    "username_claimed": "blue"
  },
  "igromania": {
    "errorMsg": "\u041f\u043e\u043b\u044c\u0437\u043e\u0432\u0430\u0442\u0435\u043b\u044c \u043d\u0435 \u0437\u0430\u0440\u0435\u0433\u0438\u0441\u0442\u0440\u0438\u0440\u043e\u0432\u0430\u043d \u0438 \u043d\u0435 \u0438\u043c\u0435\u0435\u0442 \u043f\u0440\u043e\u0444\u0438\u043b\u044f \u0434\u043b\u044f \u043f\u0440\u043e\u0441\u043c\u043e\u0442\u0440\u0430.",
    "errorType": "message",
    "url": "http://forum.igromania.ru/member.php?username={}",
    "urlMain": "http://forum.igromania.ru/",
    "username_claimed": "blue"
  },
  "interpals": {
    "errorMsg": "The requested user does not exist or is inactive",
    "errorType": "message",
    "url": "https://www.interpals.net/{}",
    "urlMain": "https://www.interpals.net/",
    "username_claimed": "blue"
  },
  "irecommend": {
    "errorType": "status_code",
    "url": "https://irecommend.ru/users/{}",
    "urlMain": "https://irecommend.ru/",
    "username_claimed": "blue"
  },
  "jbzd.com.pl": {
    "errorType": "status_code",
    "url": "https://jbzd.com.pl/uzytkownik/{}",
    "urlMain": "https://jbzd.com.pl/",
    "username_claimed": "blue"
  },
  "jeuxvideo": {
    "errorType": "status_code",
    "request_method": "GET",
    "url": "https://www.jeuxvideo.com/profil/{}",
    "urlMain": "https://www.jeuxvideo.com",
    "urlProbe": "https://www.jeuxvideo.com/profil/{}?mode=infos",
    "username_claimed": "adam"
  },
  "kofi": {
    "errorType": "response_url",
    "errorUrl": "https://ko-fi.com/art?=redirect",
    "url": "https://ko-fi.com/{}",
    "urlMain": "https://ko-fi.com",
    "username_claimed": "yeahkenny"
  },
  "kwork": {
    "errorType": "status_code",
    "url": "https://kwork.ru/user/{}",
    "urlMain": "https://www.kwork.ru/",
    "username_claimed": "blue"
  },
  "last.fm": {
    "errorType": "status_code",
    "url": "https://last.fm/user/{}",
    "urlMain": "https://last.fm/",
    "username_claimed": "blue"
  },
  "leasehackr": {
    "errorType": "status_code",
    "url": "https://forum.leasehackr.com/u/{}/summary/",
    "urlMain": "https://forum.leasehackr.com/",
    "username_claimed": "adam"
  },
  "livelib": {
    "errorType": "status_code",
    "url": "https://www.livelib.ru/reader/{}",
    "urlMain": "https://www.livelib.ru/",
    "username_claimed": "blue"
  },
  "mastodon.cloud": {
    "errorType": "status_code",
    "url": "https://mastodon.cloud/@{}",
    "urlMain": "https://mastodon.cloud/",
    "username_claimed": "TheAdmin"
  },
  "mastodon.social": {
    "errorType": "status_code",
    "url": "https://mastodon.social/@{}",
    "urlMain": "https://chaos.social/",
    "username_claimed": "Gargron"
  },
  "mastodon.technology": {
    "errorType": "status_code",
    "url": "https://mastodon.technology/@{}",
    "urlMain": "https://mastodon.xyz/",
    "username_claimed": "ashfurrow"
  },
  "mastodon.xyz": {
    "errorType": "status_code",
    "url": "https://mastodon.xyz/@{}",
    "urlMain": "https://mastodon.xyz/",
    "username_claimed": "TheKinrar"
  },
  "mercadolivre": {
    "errorType": "status_code",
    "url": "https://www.mercadolivre.com.br/perfil/{}",
    "urlMain": "https://www.mercadolivre.com.br",
    "username_claimed": "blue"
  },
  "minds": {
    "errorMsg": "\"valid\":true",
    "errorType": "message",
    "url": "https://www.minds.com/{}/",
    "urlMain": "https://www.minds.com",
    "urlProbe": "https://www.minds.com/api/v3/register/validate?username={}",
    "username_claimed": "john"
  },
  "moikrug": {
    "errorType": "status_code",
    "url": "https://moikrug.ru/{}",
    "urlMain": "https://moikrug.ru/",
    "username_claimed": "blue"
  },
  "mstdn.io": {
    "errorType": "status_code",
    "url": "https://mstdn.io/@{}",
    "urlMain": "https://mstdn.io/",
    "username_claimed": "blue"
  },
  "nairaland.com": {
    "errorType": "status_code",
    "url": "https://www.nairaland.com/{}",
    "urlMain": "https://www.nairaland.com/",
    "username_claimed": "red"
  },
  "nnRU": {
    "errorType": "status_code",
    "regexCheck": "^[a-zA-Z0-9@_-]$",
    "url": "https://{}.www.nn.ru/",
    "urlMain": "https://www.nn.ru/",
    "username_claimed": "blue"
  },
  "note": {
    "errorType": "status_code",
    "url": "https://note.com/{}",
    "urlMain": "https://note.com/",
    "username_claimed": "blue"
  },
  "npm": {
    "errorType": "status_code",
    "url": "https://www.npmjs.com/~{}",
    "urlMain": "https://www.npmjs.com/",
    "username_claimed": "kennethsweezy"
  },
  "opennet": {
    "errorMsg": "\u0418\u043c\u044f \u0443\u0447\u0430\u0441\u0442\u043d\u0438\u043a\u0430 \u043d\u0435 \u043d\u0430\u0439\u0434\u0435\u043d\u043e",
    "errorType": "message",
    "regexCheck": "^[^-]*$",
    "url": "https://www.opennet.ru/~{}",
    "urlMain": "https://www.opennet.ru/",
    "username_claimed": "anonismus"
  },
  "osu!": {
    "errorType": "status_code",
    "url": "https://osu.ppy.sh/users/{}",
    "urlMain": "https://osu.ppy.sh/",
    "username_claimed": "blue"
  },
  "phpRU": {
    "errorMsg": "\u0423\u043a\u0430\u0437\u0430\u043d\u043d\u044b\u0439 \u043f\u043e\u043b\u044c\u0437\u043e\u0432\u0430\u0442\u0435\u043b\u044c \u043d\u0435 \u043d\u0430\u0439\u0434\u0435\u043d. \u041f\u043e\u0436\u0430\u043b\u0443\u0439\u0441\u0442\u0430, \u0432\u0432\u0435\u0434\u0438\u0442\u0435 \u0434\u0440\u0443\u0433\u043e\u0435 \u0438\u043c\u044f.",
    "errorType": "message",
    "url": "https://php.ru/forum/members/?username={}",
    "urlMain": "https://php.ru/forum/",
    "username_claimed": "apple"
  },
  "pikabu": {
    "errorType": "status_code",
    "url": "https://pikabu.ru/@{}",
    "urlMain": "https://pikabu.ru/",
    "username_claimed": "blue"
  },
  "pr0gramm": {
    "errorType": "status_code",
    "url": "https://pr0gramm.com/user/{}",
    "urlMain": "https://pr0gramm.com/",
    "urlProbe": "https://pr0gramm.com/api/profile/info?name={}",
    "username_claimed": "cha0s"
  },
  "prog.hu": {
    "errorType": "response_url",
    "errorUrl": "https://prog.hu/azonosito/info/{}",
    "url": "https://prog.hu/azonosito/info/{}",
    "urlMain": "https://prog.hu/",
    "username_claimed": "Sting"
  },
  "queer.af": {
    "errorType": "status_code",
    "url": "https://queer.af/@{}",
    "urlMain": "https://queer.af/",
    "username_claimed": "erincandescent"
  },
  "satsisRU": {
    "errorType": "status_code",
    "url": "https://satsis.info/user/{}",
    "urlMain": "https://satsis.info/",
    "username_claimed": "red"
  },
  "sessionize": {
    "errorType": "status_code",
    "url": "https://sessionize.com/{}",
    "urlMain": "https://sessionize.com/",
    "username_claimed": "jason-mayes"
  },
  "skyrock": {
    "errorType": "status_code",
    "regexCheck": "^[a-zA-Z0-9@_-]$",
    "url": "https://{}.skyrock.com/",
    "urlMain": "https://skyrock.com/",
    "username_claimed": "red"
  },
  "social.tchncs.de": {
    "errorType": "status_code",
    "url": "https://social.tchncs.de/@{}",
    "urlMain": "https://social.tchncs.de/",
    "username_claimed": "Milan"
  },
  "spletnik": {
    "errorType": "status_code",
    "url": "https://spletnik.ru/user/{}",
    "urlMain": "https://spletnik.ru/",
    "username_claimed": "blue"
  },
  "svidbook": {
    "errorType": "status_code",
    "url": "https://www.svidbook.ru/user/{}",
    "urlMain": "https://www.svidbook.ru/",
    "username_claimed": "green"
  },
  "threads": {
    "errorMsg": "<title>Threads</title>",
    "errorType": "message",
    "headers": {
      "Sec-Fetch-Mode": "navigate"
    },
    "url": "https://www.threads.net/@{}",
    "urlMain": "https://www.threads.net/",
    "username_claimed": "zuck"
  },
  "toster": {
    "errorType": "status_code",
    "url": "https://www.toster.ru/user/{}/answers",
    "urlMain": "https://www.toster.ru/",
    "username_claimed": "adam"
  },
  "uid": {
    "errorType": "status_code",
    "url": "http://uid.me/{}",
    "urlMain": "https://uid.me/",
    "username_claimed": "blue"
  },
  "wiki.vg": {
    "errorType": "status_code",
    "url": "https://wiki.vg/User:{}",
    "urlMain": "https://wiki.vg/",
    "username_claimed": "Auri"
  },
  "xHamster": {
    "errorType": "status_code",
    "isNSFW": true,
    "url": "https://xhamster.com/users/{}",
    "urlMain": "https://xhamster.com",
    "urlProbe": "https://xhamster.com/users/{}?old_browser=true",
    "username_claimed": "blue"
  },
  "znanylekarz.pl": {
    "errorType": "status_code",
    "url": "https://www.znanylekarz.pl/{}",
    "urlMain": "https://znanylekarz.pl",
    "username_claimed": "janusz-nowak"
  }
}
"#.into()
}