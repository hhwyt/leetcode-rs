struct Solution;

impl Solution {
    fn word_break_recursive(s: &str, word_dict: &Vec<String>) -> bool {
        if s.is_empty() {
            return true;
        }

        for word in word_dict {
            if s.starts_with(word) && Self::word_break_recursive(&s[word.len()..], word_dict) {
                return true;
            }
        }

        false
    }
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        return Self::word_break_recursive(&s, &word_dict);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal() {
        let s = "leetcode".to_owned();
        let word_dict = vec!["leet".to_owned(), "code".to_owned()];
        let expected = true;
        assert_eq!(Solution::word_break(s, word_dict), expected);
    }

    #[test]
    fn test_duplicate() {
        let s = "applepenapple".to_owned();
        let word_dict = vec!["apple".to_owned(), "pen".to_owned()];
        let expected = true;
        assert_eq!(Solution::word_break(s, word_dict), expected);
    }

    #[test]
    fn test_false() {
        let s = "catsandog".to_owned();
        let word_dict = vec!["cats".to_owned(), "dog".to_owned(), "sand".to_owned(), "and".to_owned(), "cat".to_owned()];
        let expected = false;
        assert_eq!(Solution::word_break(s, word_dict), expected);
    }

    #[test]
    fn test_1() {
        let s = "dogs".to_owned();
        let word_dict = vec!["dog".to_owned(), "s".to_owned(), "gs".to_owned()];
        let expected = true;
        assert_eq!(Solution::word_break(s, word_dict), expected);
    }

    #[test]
    fn test_2() {
        let s = "fohhemkkaecojceoaejkkoedkofhmohkcjmkggcmnami".to_owned();
        let word_dict = vec!["kfomka".to_owned(), "hecagbngambii".to_owned(), "anobmnikj".to_owned(), "c".to_owned(), "nnkmfelneemfgcl".to_owned(), "ah".to_owned(), "bgomgohl".to_owned(), "lcbjbg".to_owned(), "ebjfoiddndih".to_owned(), "hjknoamjbfhckb".to_owned(), "eioldlijmmla".to_owned(), "nbekmcnakif".to_owned(), "fgahmihodolmhbi".to_owned(), "gnjfe".to_owned(), "hk".to_owned(), "b".to_owned(), "jbfgm".to_owned(), "ecojceoaejkkoed".to_owned(), "cemodhmbcmgl".to_owned(), "j".to_owned(), "gdcnjj".to_owned(), "kolaijoicbc".to_owned(), "liibjjcini".to_owned(), "lmbenj".to_owned(), "eklingemgdjncaa".to_owned(), "m".to_owned(), "hkh".to_owned(), "fblb".to_owned(), "fk".to_owned(), "nnfkfanaga".to_owned(), "eldjml".to_owned(), "iejn".to_owned(), "gbmjfdooeeko".to_owned(), "jafogijka".to_owned(), "ngnfggojmhclkjd".to_owned(), "bfagnfclg".to_owned(), "imkeobcdidiifbm".to_owned(), "ogeo".to_owned(), "gicjog".to_owned(), "cjnibenelm".to_owned(), "ogoloc".to_owned(), "edciifkaff".to_owned(), "kbeeg".to_owned(), "nebn".to_owned(), "jdd".to_owned(), "aeojhclmdn".to_owned(), "dilbhl".to_owned(), "dkk".to_owned(), "bgmck".to_owned(), "ohgkefkadonafg".to_owned(), "labem".to_owned(), "fheoglj".to_owned(), "gkcanacfjfhogjc".to_owned(), "eglkcddd".to_owned(), "lelelihakeh".to_owned(), "hhjijfiodfi".to_owned(), "enehbibnhfjd".to_owned(), "gkm".to_owned(), "ggj".to_owned(), "ag".to_owned(), "hhhjogk".to_owned(), "lllicdhihn".to_owned(), "goakjjnk".to_owned(), "lhbn".to_owned(), "fhheedadamlnedh".to_owned(), "bin".to_owned(), "cl".to_owned(), "ggjljjjf".to_owned(), "fdcdaobhlhgj".to_owned(), "nijlf".to_owned(), "i".to_owned(), "gaemagobjfc".to_owned(), "dg".to_owned(), "g".to_owned(), "jhlelodgeekj".to_owned(), "hcimohlni".to_owned(), "fdoiohikhacgb".to_owned(), "k".to_owned(), "doiaigclm".to_owned(), "bdfaoncbhfkdbjd".to_owned(), "f".to_owned(), "jaikbciac".to_owned(), "cjgadmfoodmba".to_owned(), "molokllh".to_owned(), "gfkngeebnggo".to_owned(), "lahd".to_owned(), "n".to_owned(), "ehfngoc".to_owned(), "lejfcee".to_owned(), "kofhmoh".to_owned(), "cgda".to_owned(), "de".to_owned(), "kljnicikjeh".to_owned(), "edomdbibhif".to_owned(), "jehdkgmmofihdi".to_owned(), "hifcjkloebel".to_owned(), "gcghgbemjege".to_owned(), "kobhhefbbb".to_owned(), "aaikgaolhllhlm".to_owned(), "akg".to_owned(), "kmmikgkhnn".to_owned(), "dnamfhaf".to_owned(), "mjhj".to_owned(), "ifadcgmgjaa".to_owned(), "acnjehgkflgkd".to_owned(), "bjj".to_owned(), "maihjn".to_owned(), "ojakklhl".to_owned(), "ign".to_owned(), "jhd".to_owned(), "kndkhbebgh".to_owned(), "amljjfeahcdlfdg".to_owned(), "fnboolobch".to_owned(), "gcclgcoaojc".to_owned(), "kfokbbkllmcd".to_owned(), "fec".to_owned(), "dljma".to_owned(), "noa".to_owned(), "cfjie".to_owned(), "fohhemkka".to_owned(), "bfaldajf".to_owned(), "nbk".to_owned(), "kmbnjoalnhki".to_owned(), "ccieabbnlhbjmj".to_owned(), "nmacelialookal".to_owned(), "hdlefnbmgklo".to_owned(), "bfbblofk".to_owned(), "doohocnadd".to_owned(), "klmed".to_owned(), "e".to_owned(), "hkkcmbljlojkghm".to_owned(), "jjiadlgf".to_owned(), "ogadjhambjikce".to_owned(), "bglghjndlk".to_owned(), "gackokkbhj".to_owned(), "oofohdogb".to_owned(), "leiolllnjj".to_owned(), "edekdnibja".to_owned(), "gjhglilocif".to_owned(), "ccfnfjalchc".to_owned(), "gl".to_owned(), "ihee".to_owned(), "cfgccdmecem".to_owned(), "mdmcdgjelhgk".to_owned(), "laboglchdhbk".to_owned(), "ajmiim".to_owned(), "cebhalkngloae".to_owned(), "hgohednmkahdi".to_owned(), "ddiecjnkmgbbei".to_owned(), "ajaengmcdlbk".to_owned(), "kgg".to_owned(), "ndchkjdn".to_owned(), "heklaamafiomea".to_owned(), "ehg".to_owned(), "imelcifnhkae".to_owned(), "hcgadilb".to_owned(), "elndjcodnhcc".to_owned(), "nkjd".to_owned(), "gjnfkogkjeobo".to_owned(), "eolega".to_owned(), "lm".to_owned(), "jddfkfbbbhia".to_owned(), "cddmfeckheeo".to_owned(), "bfnmaalmjdb".to_owned(), "fbcg".to_owned(), "ko".to_owned(), "mojfj".to_owned(), "kk".to_owned(), "bbljjnnikdhg".to_owned(), "l".to_owned(), "calbc".to_owned(), "mkekn".to_owned(), "ejlhdk".to_owned(), "hkebdiebecf".to_owned(), "emhelbbda".to_owned(), "mlba".to_owned(), "ckjmih".to_owned(), "odfacclfl".to_owned(), "lgfjjbgookmnoe".to_owned(), "begnkogf".to_owned(), "gakojeblk".to_owned(), "bfflcmdko".to_owned(), "cfdclljcg".to_owned(), "ho".to_owned(), "fo".to_owned(), "acmi".to_owned(), "oemknmffgcio".to_owned(), "mlkhk".to_owned(), "kfhkndmdojhidg".to_owned(), "ckfcibmnikn".to_owned(), "dgoecamdliaeeoa".to_owned(), "ocealkbbec".to_owned(), "kbmmihb".to_owned(), "ncikad".to_owned(), "hi".to_owned(), "nccjbnldneijc".to_owned(), "hgiccigeehmdl".to_owned(), "dlfmjhmioa".to_owned(), "kmff".to_owned(), "gfhkd".to_owned(), "okiamg".to_owned(), "ekdbamm".to_owned(), "fc".to_owned(), "neg".to_owned(), "cfmo".to_owned(), "ccgahikbbl".to_owned(), "khhoc".to_owned(), "elbg".to_owned(), "cbghbacjbfm".to_owned(), "jkagbmfgemjfg".to_owned(), "ijceidhhajmja".to_owned(), "imibemhdg".to_owned(), "ja".to_owned(), "idkfd".to_owned(), "ndogdkjjkf".to_owned(), "fhic".to_owned(), "ooajkki".to_owned(), "fdnjhh".to_owned(), "ba".to_owned(), "jdlnidngkfffbmi".to_owned(), "jddjfnnjoidcnm".to_owned(), "kghljjikbacd".to_owned(), "idllbbn".to_owned(), "d".to_owned(), "mgkajbnjedeiee".to_owned(), "fbllleanknmoomb".to_owned(), "lom".to_owned(), "kofjmmjm".to_owned(), "mcdlbglonin".to_owned(), "gcnboanh".to_owned(), "fggii".to_owned(), "fdkbmic".to_owned(), "bbiln".to_owned(), "cdjcjhonjgiagkb".to_owned(), "kooenbeoongcle".to_owned(), "cecnlfbaanckdkj".to_owned(), "fejlmog".to_owned(), "fanekdneoaammb".to_owned(), "maojbcegdamn".to_owned(), "bcmanmjdeabdo".to_owned(), "amloj".to_owned(), "adgoej".to_owned(), "jh".to_owned(), "fhf".to_owned(), "cogdljlgek".to_owned(), "o".to_owned(), "joeiajlioggj".to_owned(), "oncal".to_owned(), "lbgg".to_owned(), "elainnbffk".to_owned(), "hbdi".to_owned(), "femcanllndoh".to_owned(), "ke".to_owned(), "hmib".to_owned(), "nagfahhljh".to_owned(), "ibifdlfeechcbal".to_owned(), "knec".to_owned(), "oegfcghlgalcnno".to_owned(), "abiefmjldmln".to_owned(), "mlfglgni".to_owned(), "jkofhjeb".to_owned(), "ifjbneblfldjel".to_owned(), "nahhcimkjhjgb".to_owned(), "cdgkbn".to_owned(), "nnklfbeecgedie".to_owned(), "gmllmjbodhgllc".to_owned(), "hogollongjo".to_owned(), "fmoinacebll".to_owned(), "fkngbganmh".to_owned(), "jgdblmhlmfij".to_owned(), "fkkdjknahamcfb".to_owned(), "aieakdokibj".to_owned(), "hddlcdiailhd".to_owned(), "iajhmg".to_owned(), "jenocgo".to_owned(), "embdib".to_owned(), "dghbmljjogka".to_owned(), "bahcggjgmlf".to_owned(), "fb".to_owned(), "jldkcfom".to_owned(), "mfi".to_owned(), "kdkke".to_owned(), "odhbl".to_owned(), "jin".to_owned(), "kcjmkggcmnami".to_owned(), "kofig".to_owned(), "bid".to_owned(), "ohnohi".to_owned(), "fcbojdgoaoa".to_owned(), "dj".to_owned(), "ifkbmbod".to_owned(), "dhdedohlghk".to_owned(), "nmkeakohicfdjf".to_owned(), "ahbifnnoaldgbj".to_owned(), "egldeibiinoac".to_owned(), "iehfhjjjmil".to_owned(), "bmeimi".to_owned(), "ombngooicknel".to_owned(), "lfdkngobmik".to_owned(), "ifjcjkfnmgjcnmi".to_owned(), "fmf".to_owned(), "aoeaa".to_owned(), "an".to_owned(), "ffgddcjblehhggo".to_owned(), "hijfdcchdilcl".to_owned(), "hacbaamkhblnkk".to_owned(), "najefebghcbkjfl".to_owned(), "hcnnlogjfmmjcma".to_owned(), "njgcogemlnohl".to_owned(), "ihejh".to_owned(), "ej".to_owned(), "ofn".to_owned(), "ggcklj".to_owned(), "omah".to_owned(), "hg".to_owned(), "obk".to_owned(), "giig".to_owned(), "cklna".to_owned(), "lihaiollfnem".to_owned(), "ionlnlhjckf".to_owned(), "cfdlijnmgjoebl".to_owned(), "dloehimen".to_owned(), "acggkacahfhkdne".to_owned(), "iecd".to_owned(), "gn".to_owned(), "odgbnalk".to_owned(), "ahfhcd".to_owned(), "dghlag".to_owned(), "bchfe".to_owned(), "dldblmnbifnmlo".to_owned(), "cffhbijal".to_owned(), "dbddifnojfibha".to_owned(), "mhh".to_owned(), "cjjol".to_owned(), "fed".to_owned(), "bhcnf".to_owned(), "ciiibbedklnnk".to_owned(), "ikniooicmm".to_owned(), "ejf".to_owned(), "ammeennkcdgbjco".to_owned(), "jmhmd".to_owned(), "cek".to_owned(), "bjbhcmda".to_owned(), "kfjmhbf".to_owned(), "chjmmnea".to_owned(), "ifccifn".to_owned(), "naedmco".to_owned(), "iohchafbega".to_owned(), "kjejfhbco".to_owned(), "anlhhhhg".to_owned()];
        let expected = true;
        assert_eq!(Solution::word_break(s, word_dict), expected);
    }
}
