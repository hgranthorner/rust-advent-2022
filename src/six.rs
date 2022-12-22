pub const SAMPLES: [(&str, usize, usize); 5] = [
    ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19),
    ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23),
    ("nppdvjthqldpwncqszvftbrmjlhg", 6, 23),
    ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29),
    ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26),
];

pub struct Journal<const COUNT: usize> {
    pub records: [char; COUNT],
}

impl<const COUNT: usize> Journal<COUNT> {
    pub fn new(records: [char; COUNT]) -> Self {
        Self { records }
    }
    pub fn add(&mut self, c: char) {
        for i in 1..self.records.len() {
            self.records[i - 1] = self.records[i];
        }
        self.records[self.records.len() - 1] = c;
    }
    pub fn duplicates(&self) -> bool {
        for (i, c) in self.records.into_iter().enumerate() {
            if i == self.records.len() - 1 {
                return false;
            }

            for j in (i + 1)..self.records.len() {
                let other_c = self.records[j];
                if c == other_c {
                    return true;
                }
            }
        }

        panic!("Should never reach here!");
    }
}

pub fn solve_first(input: &str) -> usize {
    let mut cs = input.chars().enumerate();

    let journal = Journal::new([
        '-',
        cs.next().unwrap().1,
        cs.next().unwrap().1,
        cs.next().unwrap().1,
    ]);
    if let Some(value) = check_for_duplicates(cs, journal) {
        return value;
    }

    panic!("Failed to find marker!");
}

fn check_for_duplicates<const COUNT: usize>(cs: std::iter::Enumerate<std::str::Chars>, mut journal: Journal<COUNT>) -> Option<usize> {
    for (i, c) in cs {
        journal.add(c);
        if !journal.duplicates() {
            return Some(i + 1);
        }
    }
    None
}

pub fn solve_second(input: &str) -> usize {
    let mut cs = input.chars().enumerate();
    let mut journal = Journal::new(['-'; 14]);
    for _ in 0..14 {
        journal.add(cs.next().unwrap().1);
    }

    if let Some(value) = check_for_duplicates(cs, journal) {
        return value;
    }

    panic!("Failed to find marker!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_first() {
        for (sample, expected, _) in SAMPLES {
            assert_eq!(expected, solve_first(sample));
        }
        assert_eq!(1134, solve_first(INPUT));
    }

    #[test]
    fn test_solve_second() {
        for (sample, _, expected) in SAMPLES {
            assert_eq!(expected, solve_second(sample));
        }
        assert_eq!(2263, solve_second(INPUT));
    }
}

pub const INPUT: &str = "tnmmpfmfzmmnsmsjmjjbvvhnhzzfmmgpmgpgbgnnwffjhffzqqmzzbnbssrqqrnnhsnngsszsqzszhzfhzfzwzfzrrmhmghgwhhjjqwqttwhttjllrtrtzzcfzfgzznfznfzfnnbddvmvzmmfsmfsmfffhlfldlqqrnrznnhmmgqqzhhmjhmhppqbpbbngnlldvvdqvvrtrdrtrnttnppfllrbbrprpnpdplpmllhwwddqpdprddzzfccqpcqpcpcbbdhdjdjwjcwcctdcttzgzmmscmsmdmttwhwzhhnjhnhlhvhlvlglpgpmmjmgmrgrddmwddjfftfwflfslffqtfqttpftppflfmmhvhvcvbvhbhggpbgbppvdpvpvfppbwwsnnhphllbdbnbvbmvvzffvsffdldmlmtmccnlnbnjbnjnhhbfhhgzzlwlfflzffdccggdcgcjjhffjfgfgcczjccvwcvvqgvvqvllqzqmqllhjjqnqggttsdddjgdjgjzzrgrfrbrssrgrgdgrgbbssmdsdfddsndnsdnsdnnmqqsspqqmrqqpmmsjmmszzqvqrvrzznnjdndtntfnttgtctqtwwnwswrrthrttsdttlhlvvdzzgqgttnppjpljplpgpvgvqqvppzmmqggtjgtgstslltjltjjgcjcmjmsshvvtppgmmlslqqshqshsllbggfpgffdsdgssncchctcwwtllgqlqblqlqvvmsvmmwnnzppqllsttgmttftvfvjjrzzswzzjvzjzljjchcshcscbbrdbrbcrrnvvtctntvtvbvjvqjqggsrspsprrbgghdghhmwwldldzdttrvrnrfftqtftrrdsszlzvvbtbffftzzrzqrrhjhghhwbhhsjsfsttdjdjnjhjmjpmplplrrdjdcdjdbjblllbqlqdlqlpqptppdhhqmqfqhqhchwwqjqfjqfqhqshsmswsbbvssdspdpsdssstntltrrgnnmttgmmsjjrlrnlrnrnwrwfwlfltlzllcjcmjcjpjhphcpcwppmvmjjzbzvbbfnfcflfddntddbmmmhnnsrnrrvdvnvcvwvcwvwrrqwqccqmmswmmjrjmmwjmjfjhwrtbjzdvlgrjmvzfmhcqsncvlhzzncjlbvcwrdwjmqjcnptqslvfzpsvltgzsvjdsjrppdrmqrbqwhddfhnftfblspsrhtdtjwdnhbcbtlwlvccsfscvczzrrqmwbwbdmwgzqntvflppqvppwrhnvtlsbzqglhsfdgssqzdtjdpwrrhbnbtwhhnmnlwfwlqffjjrndbpwwsvdrhddbjnnqzmtpvvtwbcpndjzlhcfrrdvmljswjzvmfqcdsgqwclqshwrmblszdvsnrpdgnllmlchzdjlrrpndmmgddjqgjqrhwfbwddqdfbvptrmzhtsqfsfswpnvmtswqprjhbzvntgrlzthhnqbtpplqpvcfnpgdtbhqbhflltbbtmmhcwztslmpznttmssclhmnbsbrwlblrbsdfmnpqbwwmsncvzmpqwhzjgcgdrzvglgdtswmstdhrprdjfmqtjlmplbjtzcgnrwpdvpfjjfwjfnnpmdtwtqsgfndngsbmcwjtglqwtfrclbczfcmjtgcwszhzrbcphrhwmhcwghjznzthnwpljjltdlvqtffsrbmwcsvrdmqqggbznnlzbbqtgspqvnjpbdhtzmgttrcwwszwpgdrcnfqtgrgqdrctlzwtdwqppbhnwgldnqltznnfpbfqtgmmwpcqnndbgmrrtgtvnmlfcwsldchjnnqfrhpzwtclrzftsqllgvpqbgmfjdhqjttwcvbpvfqsvhbhhtwnqnbgndbtzhcvgglbhghbzrbrmdllmgfgttqmhtdnwrpwllhnghrjctrbzrcpnjnctvmrlpjhftnfbczrjrnnbqplplcrbngbhvmmvcffmgvbhjzbhcmtwmwgmjmwjvvlqfldswpntjnsjvmdlbzqqlgbwspwvmnwtwjbczmwplrhmjgsppnmtwmvsfwnsgddgwqcvpftcpzrhpldnwmcjgtjmljjbcmjcqdbwczndnjnjgrmtjrqnnjndzqdqpcgdqptdbrqftnwrgqmrzrvsfmmmbpltlncvtgrjfjmvtgwqphczwjhdrdwtfvgztbhrndvpcbgfjfvmrrljwrvcrtdmtjndfnwgcnfrzgsnjpztbwwsbvqfnpjctgrhsflhnzbbsfqbnmtnvrmjzsbjfndvttpvpfjhqntflgbfnzcclcwmhbsgqfjdcgsvrhtstspfzgvgglgddqmclsmzgzgtncdsfmwdvtcsgwvbzjvclwppqdjgfcrcbzcwbdhrnssjbmnmfmwthdrnmlfhqlddwqrdhsdvdcsmcgjsgcmpnhlbnqftpdjswtmpbznlcrhtswgnmwjcdfmljdngzfsmlzjjnzmfzshmztdbdmcqwmlvcrzgpmbjqcghclwvdbrhgvwqchnndftnrtptmctdlhmfjvpzrpccddfpcdwmzqfhnsqzrvwblzfhcjdcjfctczwqrcbjnrpdcbbnsgnlvqqmnsfgsqschjlbzhhsrbvdbfrhvsgrlzwncgwpdbvmblgzbwbcbgqfwmdmgcrbbjfcvmqgztqpptdhwmvmsdqwplpgcjzgqzdrftzhqbltvhrmlrfffcgfpqzwrrbbtlsjgmtbjvtnmhwdpjptjwfwgjgvbfqwmflrrqzlzdcmtlnptdrpcpdnswcfscnndnrfbgwvvncdjgsdpbwptdtvrqlmrhmvvcwblhhzbjdpsbszhrftfbcgwhwrgglnjzqdhcqnvlhgqjhnddvrslhntssptsbhmqwwqqnbvfmcbgpvgjbrttnvlljdbtfplgmbwtcbcdtqdpqqdvhbmpmtszwpzblcfrtznhhtcljtdlhjdbnlhvwgjsmgvrslrfwnmzwlstpgltvrgnpdqztvfnvdhdtwwqdfsmtpbpdclsbnwcgjzchjcsjmvhbjshmjjlpgdzcgbmmchwmcsddsvhsnpqtcpnhqnbvwgwqhtjbqncgwwftnrzsbsjtvqmjzqvvncmncwflcfpcjqgdtbsmjzzsdjfvhnqbgjhmfgjghwscthbfmbndltbqzwpqtmrswvprpmgwqnqpfnmffrpdlpfqmhrthppzvzwbrtjvwvjndsqdlqtbpqwfcttggnjmcqqnmjwfhfjgcvlnmtlgbdvmctzlwbfgnflwtsflgnfbnfbhhdgjctzvvmrhdsmvmmtnqwtszmqcpsbrqrgjfrzctcbzmtdlhwjtfdqbtthdnqcrpwrhcrvjstbhpltvgmvpmvfjstgzjsgzprzcqzqztvvdcnrrqwrhddcrhhncdrlwzwqlnbbzcfmqtnwgfdscmrbwnbldlfrqchzdnlnmwncgrzdclnvcvplgwjsbzmbnnsdrsfhrlssvncnwmcrjdjbjpdtrrvlnbjvspfqbwdpcnnpjzfnmbhcdhlmdgbpvbzmfltzstnznfctcdzhbfsvnfbsjqzmwfllhtrsfghlrpjgrgzgchlwrdmqzbrncsvnwhfqmwjbnvjctzphcsftqsbmwntgvjqhhvwndvmfmjhhhmfdvrlhpvzmmhrbhbddqbdmgqqsvddsswmzqcjmvhztfqpchzpwhdshzjlmbmnsgzqhbnmrshwvtmgmgndtddpfwsjrrjdhncdhtlczdvlbvqplttnzrblthlcffdtfsdtpwzdgbldvnsttvpzmbgnqddrszftcpwrgmfzhjjvghpntmzcttcsnrjnfpqzqqqljhzlrpgwngllqjwnwfcsphqplgbzmfqfgbfsqpsrntszqbcqnhctsnbfshmlbwfflrwwsjwqwfqlgnftdwmctmclwjhjhbsspqldlshbmpbgrftpnbpsqldhrrbdqwfwvfhclrlfdjfmzgmptdjdcsplcspznfjrfhtsjndwpslrdgnllllwqjgznrhswfssdlvdpmwwgmstqbhfmdhtzvzzvhwzbrrvvsl";
