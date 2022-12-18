use std::collections::HashSet;
use itertools::Itertools;

pub const SAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

pub fn solve_first(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        let half = line.len() / 2;
        let fst = &line[..half];
        let snd = &line[half..];

        let mut set_one = HashSet::new();
        set_one.extend(fst.chars());

        let mut set_two = HashSet::new();
        set_two.extend(snd.chars());

        let mut inter = set_one.intersection(&set_two);
        let c = *inter.nth(0).unwrap();

        if 'a' <= c && c <= 'z' {
            sum += c as usize - 'a' as usize + 1;
        }

        if 'A' <= c && c <= 'Z' {
            sum += c as usize - 'A' as usize + 27;
        }
    }

    sum
}

pub fn solve_second(input: &str) -> usize {
    let mut sum = 0;
    for mut line in &input.lines().chunks(3) {
        let fst = line.next().unwrap();
        let snd = line.next().unwrap();
        let thd = line.next().unwrap();

        let mut set_one = HashSet::new();
        set_one.extend(fst.chars());

        let mut set_two = HashSet::new();
        set_two.extend(snd.chars());

        let mut set_three = HashSet::new();
        set_three.extend(thd.chars());

        let inter = set_one.intersection(&set_two);
        let mut inter_set = HashSet::new();
        inter_set.extend(inter);

        let mut inter = inter_set.intersection(&set_three);
        let c = *inter.nth(0).unwrap();

        if 'a' <= c && c <= 'z' {
            sum += c as usize - 'a' as usize + 1;
        }

        if 'A' <= c && c <= 'Z' {
            sum += c as usize - 'A' as usize + 27;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_first() {
        assert_eq!(157, solve_first(SAMPLE));
        assert_eq!(7581, solve_first(INPUT));
    }

    #[test]
    fn solves_second() {
        assert_eq!(70, solve_second(SAMPLE));
        assert_eq!(2525, solve_second(INPUT));
    }
}

pub const INPUT: &str = "LHLRlCCvCLVgHPfCHtVjBGrBDNzWFBsBGBfscGsD
nQwbnwwpbrJBrNWB
hmnSdSdQpTpdnlPdvddPNglLjH
RZhwpDsNqVmQClwl
TLJfLTPqcvTrvvLMLMlVzzvVVQQtmQCmtzmV
MJjccdfTMcbqjNSRSZsSDZ
LLrNNqCTCwLTttwcNctqFGmRBSBjzjbSzbBbjNbzjB
GnhhZQPDGdldgQmQSjpzjzQssb
gDJZPMnPnhlhJWhZntLCLcTqVMLrGVtMfM
rrBgDBGnVnffDnfQQqngJhhSRQvhhCRRRSZbRpRzwQ
NtLmcHPHMHHssFJphZpbhwpNRbbC
LJPHlmdJDgrrqrnl
nJhrcNnfrFwNhPdMQSgZSCMjQn
LjqGWsGWllRRlHVsqGGWsZZSSHQgPmHZZSPvdPCmvQ
zqqVTWjqBsTJprNbppFb
zSMgWzlgFSWFcGZlCZGlrrTc
spnQHdQmHddNmpWrpWcChccTWc
BsRsnmBQdNWsvRPzbzbLzDVSPSbVLM
lDfbffptlrJZTBJHjjBWjT
LcwwgQLgzvztwtMQGCMVCHWmnmjWnGhFHnZjmZhjhT
sCqtzsswCgccbSqrDSqbNNfN
snnnjwRRwGSSnVmhhVMhGFbgQgbzFFPPgQQmPbbgQd
qCrccvcDDcvqDZlCcrcfQNQFdsbgWzFfQddQQPgQ
ZcrvrBqBTCZnBBswjwpSRs
qSczBfBcjMZMfctsmsGmFJsmQQcQCr
wPhTLNVNGLNdGHPHwlQsnrnmnrQvHFFHQn
dLdwbNLRdgGbgTjZfDbqDWjftSzW
rZwlrtRtNtlHqVBtdqQgdq
fbwzpPwbhJzpwfTSHgdgqcJVcBjHvHdJ
LLPbhzPpTTbTshfGhPwSFWnNmMrrZZmNmZDmWNCCZs
tMHgMWMQWgFJTHsWMvJrVdlmvlSvdvlpvG
RNfZZfRttBtdlZlmmmplSS
DzzNDDRwnwwbLnMFtsMntQFM
qHqBMNqgMwHMbnGStHSbndnt
PwWZPjpfsDsDsPfPfjdbSvbWhdFSbFGSWFtr
wfpjpJcfVsspzZRRszDpwcRggLTQQBLqNqcqcggLCgNmlq
TmmFjtvFdDGjdFFJjFRDLNLHGBVcqgLcLgVBLqgV
WbWSPSwQCWrWQSrCbwNVlLlBZLBvLlvZVqPl
SfwbhrwQQbbMwCwWCrbwJRvptJfjDTDRvzTttRjp
jzqSMszqsbVVMVMgWhWCgMrpGgpB
wrwLcFQmPlFFlwLZmFGTfPvWGfPvhWWWvGgf
wRZtHFHmzNDHSqrs
NprlCgrrnrNCjplSCtljpFrFZLzzgwmVgBzBZZPwzBPQBwVL
HsDMvHTDfsfQZfZzmPWL
JcPsDGTqcTqSdrSCtnCt
cFcmfmJFtFmtlTNtLlCWTT
QPQzjRRsVsQqBqwlTlNBpLNSWDpN
QblgzRPgbgQsVvgPVQhgQqjvMnJfccnZddcGGfrFJMFGGF
PWbWmFFnPFPWbDVVmmDHDFGdGhTQdLdnTZQZZcGSGGdQ
ClzjNBlBJvlsBdcPLZdLPQjLQZ
vJMBpBzzzfNCCzCffJlzgMWDWwPDtVtmVHPMwVHD
bJjWzWFlTMjjSNBrRcBrZR
mwnwqPwnGQPCqmJmPQJPCVNcRZBRRrrNrmrcVpSrRf
PGvQQGPqvhWFWlJbDv
PNPrdmPGRJlZCrCJlGQzjRFLpFRppjgppgcj
DwfVnssbVnSWShDwsnnhBLFjFgjFBzDBjHDLpHDj
vSsMgbsTfTwwfMffnTvgNNCmrPJtCNrJrCrrtvGm
cRnRplCzccVcrwcnppVVzRCNhfhgChNJfPgHJdHDNtNCtP
WFmbLMZdLBqfJNbPTfttDD
BdMWdQsGsSsrpzrswr
llhhZzSLqlzwRrffzwzT
GvBbNjHbjjTGGHHFcsFvfRrtJQPvtRfwwfrPJD
HGbcbTTjHFNpppmLnSdplWqZ
FhwFbPwsvtRgVCgvMT
HJVHdHBWdBQSSSQnqSQLqZHtcCctppgBtRrgtMCgTprRMM
QNzZLVSLLLDGPPzPmbFs
VdTHmWCVZDTPBBWBQBFQQg
MzjMjzCjJsbJhhPz
crfGGLwwLGrtrvCtdTmdDH
wRLvLmGQLwFPBRmnLCLmGQTzNNqVNZMMVzzQbzVbNZpMVb
jsgJWjdHghsglHtWsjSfHzVNqzfpCCzqDpzDrVVrrD
jJtWWsWhtSHsSSgchthHcjHCvcFCRvwvvTTFBGvBmmGLnL
LpjWLNqWpwRWMqLRGjwJlStgbtrVgHFrGllDDrVH
SQmmTcZZvSZBTmTSzhPTddbVDhHllgFCDHtrDHgDVVDr
PzTTvznBncnfTmTTQcPdmzzMqWNfqwRLpWJsNfwLJjsSwJ
lsGdGwBsflGrfsHvHwQLdFrmPhDhCFhhjWCVmhDzmbmPhC
qtMSNNZZMpcnVzmVbCqjWjzB
ZZcgRJpBtTMNnntncwgQQdfGHHHlsQffLH
jBBtjjqfnwStBSrVVFwSVVvvWzHmcWvWbvPmPbWrbMRM
GTdNDlpJhlCvPbHgcDmgDH
JZQdQhNldLdTpGdJGdNCpLZdSBnnBFfHHswqqjffZsqsFjns
phJhDPQLDSJvpHhvDJhfrFQVRrnsslrgwrVrrRjg
mWNWqZWWZBMdCGMNCdWmWCNCsnVFTRsVnZFlsrlFwFgVsgjr
dWdjttGmNCBchJfhHvhPtvJt
CSFSFdfCzJhtSCHQFjQHQWFHRNHG
wnbrgZnwZgDLsLbwsLrsrNWQNjPZHHvPPQHNqHHvqB
LbmTLDsgggQmzmCCppdtSJtM
SzSSchCdZgHbwHSZ
GsMMmslnsfmNGNNNVVtZWdwbqQbpgWjjgWZjQm
DvlMtflGGVGthhzdvLvhrTcc
dvfVNqHlQfGRcjDczlCDnC
PsPsStLprtTTFSTLmhSVSFSsCzRRjzDnMJDCMWWDjMnMnpjW
hBFPhSBFttBhStLwmsPTtPsFHZNGfQgdgdZbdqZwdgNVvwvH
rhjcChdgjdCrjLjLLSLmLFMmFtNnnbQNNNPMbbmtQF
lwWRZDlsWzrbbQpN
rTRqsqDRRRsDRVrqDgBCdCVShjCBHchjdh
PsspltlPsmTsmbmfTPSTTCGjhJJjCnpqJJNhhJwNJh
BdrLVvgQLQVLHRZGnqRhRNdwNC
WrFVHDgDQFHVLVVDFQMLltmPsssPztwsPTzsWcmc
CBvvSzFGSGGWfFZpcHqjvjcvcqccJq
bRQwgbbhrRhdwmQbWtdjnJHqVJccJqgHVJHJcl
RRmbPrNRRPLLtmbQmbNwddCTSTzBSDBDSMFTBSSPZFFW
cppsSgNrSgwrGRdHRrwd
LzqqmCLCLWQvCzmzZwHnZZHSwvwnlDlS
hzFqFLLLFtSNVsFF
zpZcZZZdppzDLWDtJGgfGbTGGJTGcc
qhvNSClCShRrRBBWTQfgBFbgtfgg
HlHqvjqmCvCvlSSHvVdsVDzjpVjMWdwLpP
qLdsfNsTHQwnSNSBNS
gFhWzrhfbmlpmZhJWrFSvRMnwwvvpBpBSpQBMv
rgWZrbmlmbzFfglgWzGggFJLccVPqLPqtPLGcPsPHcPLTd
jTTWRCCbwJJNTHrffqNnzh
DZVmDpgGBVdcMZnqfhlNHQMlNNhl
sDcpsDZBcmgdssZcnmSWCPWRSRwJWwvFLvRwWj
hWwhgQlQQgjPhFChZVdbcJ
zHsrMPNMtDDTmbcJbccmDb
znzPzrtHtHtMzqHHrsSSLwljqfgwGggjlQjQQgQBGj
JpnRtqlJsqDJJBBNNmQmgdmRNGGmvv
hTCbTwMCwVhTWdmDgDvjWD
MDhhMSwZCbbLVhbLcDSwCwZtZznlzlnqBPBpHPPlBlHJ
CtvnvqNNDchrhFVpwftmgQgpQfwS
MbdqPWGjBjMBbwlfVgdmfSfJJS
jWzbzjWWjWMMbRbMsjzBhChrNHcNqHcrNnhssnsc
LlLJSWgWllSShRmRlBLJSVBzpTHzTTJcpTHzpTTcPpGpTr
MfMqnvbvDfbFFZDfFNjsGrRppHpZGGcGrcPprz
wfNFvtwMvbnntftjfNtVmlgmStBmlBWdQQRg
GpFRRPGWqzHwdqpzqbjjgfZptBBVMSjSfBZc
TClllrnsJvDMBgcjfmtssB
NlhNNchrNJlLvClNDrzbGRqwqqqwqPPFLGdd
qFmVtvmmVvzzFtzzGzzMNNMSSTjNJlStjSfNgf
sWrPBCnCTMsTJfSM
QTLbpnRpRppnRQdRzRZqGzFFVVRz
WGGPjFvMVNjlcQJr
bslfldbgtpSmwmSNHQhLJhcwLcQrQV
gTltCsSsssPFnDzWPTMz
hhRRhQgGrHjhRsrgqznbzncZjVVJVjncjd
DSFfNTBFSDmMSTDlFbBBdccCdJJZCbZCbW
DDLmdSmTvQQgsgvGHH
dNqNgNvFnvdZHFWnZWNBTQlPTppPGlCTpBQppq
LJrtLrsLjsGvTCTpQP
mtLhjVjMhhmVMvtJmLfhFWHnngbRRdZHngnZWZ
NzdVNzqqCtCHMMZBCGBW
psjllRjFpjpbjspFmWmWnLBmMMQMmHbm
DhsTPDRTDHpsvRjdNtzJJJdhqwcdqc
VbhRbZgRHMFhQpHd
fvlqPzmzJJqJSPsWmPTNddNFHbNFGHNTHSbc
CqzlfqrCnbrBZjBr
SNSrDZFHnTqFsFddsCmsMC
ctVthlGjfhGljcCJmcqMCqcqBB
VtjvtjhhPPtWqVPLjvqjLVNRppRTvNSnbnZRZTHRnpTD
fzsBSsNBMNMszNGGJvgjjPggzjdFPgpJ
bmrVVVrmRrrvRmwvqlbHTDgwdLFjQPJPFddwJPFggj
HHHrZqhqbTMcZBCZfvcN
tzsJsnsmBzlVqjssZZrg
MQZHfNCffpMfpGSpPvpfCGCTTVwFTlwrggjSgqFjVjTwwg
NfMGGGPZpvLDvLCGGfQHMpZRDRWchRRtBBzmJnmzmnBznb
BSRBjtNjZrsjRjjNsVBjbrMwCgGCCwCdHrlcdccGcH
DJTTJLpFnFLdJJqPLTWqLTpwHzCGccCzvvcHwMvWzggCMC
PmpTTdndmmLqfLTTLDqJVBssbbStVmjjSsZNsBZm
ddCnZvCDSgghFhbbmFVQ
JzlMcJTMMPPfJJfsMsjWlHVhLbQVlFWmHbbb
BPwwsPfsqszfFqppwTsqzpntDSnCBnDRZrSZdnDdtvZD
SllzzPplWldwLGlzbtPZZjVScnnNSjnNsqNqsc
BrCfFDJFDHBhJCChQFhCCBDcTnNVpZZcNcvQTcjvvcTcZV
RCmFHJDJhCmBmRCgDCFRpmGbWWbLPlbMWzzGttzgLMbt
vGTfsZnfvfzTjsnfzTJlwqQjwmCqqMFjFFQMlq
LHtHRVLRLNtWcmVbRrPbRcwgQwgwMwClwClwrgFpwqpw
VDPtbVLBmLbLbDDNnnnzJJfJfBfvSGBn
lpPCRVVQppzHlZgzglgF
rtfttLdLdscmGtzngPHHFHFH
LPLLhfhbTDLmPdcrcWdTcDSjjRqwwbqpRwNBNpBwjQwR
dWQfCJrwvQCfFqNwRbbzVbVVLGTR
ZpZshPMzBjGjtVMN
phpSlSlDlcZpcZPrdHCFzFzFWFDWQH
tfMMZhjLlChsdsds
PHQRMHRwpRPBMvWvPRBpPdWDslGrbscTlTGcCsGddG
FqPSvHPHPBzQRBBwwRJfVtgjzntMntjJLMtJ
VBwJvwVwNbVRdPwMgWggGMgH
jDhqflDDhrqshNhdgPGHphLg
FltsrtcFrclrNqDfqmzQJQQRBzBCvCFvBR
RZsSSJDJZLDWnGDMLD
ClbnlfmpNtmgbtmMqWdjNGjLQjLqLj
gcblTlVCnVmcPrvRPFRrZs
mbJcScmbDWLmSBzwjPRTfjmmRhpl
tFFFtGttdClHVMCHFMMwTpwNjGpzPpNRRzzpPN
gCCdvZCVHsFvJnnDSglSglDn
dSndnRRvVSpLSphfqvTgWqrzqvvw
PBFQbQbDhGfjTTFzqG
tPmCJMtDDNcMVdhhVc
QVRVHCQRmdTRqrZFCWrLZNZFbb
ncncsPnhslBRSSSbFhtbZDLMbLbtLb
lvflPcfPSsPzlJlPlcPBfHJQwRwHmqdVpRGGGdQmww
GTCGMCcGdgRnnbbbMLwmMz
DQFZzllWDDLwDJLnJpnp
qBVrNNlZfFNlWlWqfRzhgvhCqHRdCGGSvc
ZnMnGbLZfJcBcLTgWF
dHJjdzqssHHNJwCHpHtDccvtBTtvccWWrrTWSg
CmlqCNzCzHlmdsqzNzRhMhZRbZMPmRfRJQPb
LsLLrFLcFjrtmZhhmhHGhJGGhH
ffvbsbWpSBSSCCQbsSBSwwJHHvhZHHGdGVGlMlTVdZlT
pSzWNPSfCwWNPBfsFqtFLtsqRzqFgj
hwwpvjVppGpwWGLrcPjrbrrdbjdL
FBqFFMFHHsHssNHtslqtFmldnnLrPhMnccrnMzZnbLgPLz
mJBSstlJQpwGSVhC
cgJDVWsrWggpcHhMzwwPnQMWMm
SZGBjdBqBBjGjjqNGfGNNHPnRSQFzhnwmnQzQnPHMR
bjjCZddjZbZBCtLhCZhftrgJglcTlJgvTllJvDVDLg
QpRJpCFdpqTQcqSTBBGBZVjZjVjFvwVB
nnnWfnHhPDlDnlLwGjBBbVVZGbCGbP
LhhLMLMrMWMrCprTqpJQpz
dqGGZJdZbTTMFFTGJFFbMdnCHSdWcmNmcCdWSggfSW
QsjjtrzLrQwDPjrQLrCfSSnmCmHWlCgmzlNl
PQpQPjQPsBstLBPttDrjBwTZMFMZFvJFJhMhnMJqpTJJ
JMLrSvHJdJvvJfrHMJRfWzWDFPwCcWqRRRcq
ljZsZTmmtTBlpTlTjQZCtNFPVqDRwWwWWVPcDRVpFRDz
mBgQgNNTNvrvJSvgCb
DWbWtzWDfDffbsMbZMffDDLncnnCJmLVsJJJnhgcngLs
TjgNGSBRTRTQrFRjFGBVLwLnnNncCLLCCcmhPC
GgGjvgddvvWqqfdZftWH
zMmsQlMfQQMhjsmjfsmHlhncRRZnRRRJRvZWWnhccdRC
BptFtDSSrTrpgtgqqgtZtvVVdVvccVnJdVnG
qpgPqBLDNTgqBrSLpDBMJfjzmbJMHjLMfjslzH
DPgLgPhfNDRqhDFDsBTtrrrdbbztCbtf
MjGSScGVGSlJjbbrtTvdzsTq
JJwJGWMZwMlWnFFgqNQFpF
WRGDHmGqWHlrmtVVVRVqpNZvZvvvTNPMPjbPdM
BwhBwsnzwhzSfCfswFvpvTzTdpMpjvPMZNTb
LFFQgnbfChSFBhFnftRRLrttDmmRJHtlGH
MhqhRHmDdRlRlGnfZbJVsNNZDnNb
QwvzgtwvFpmjwzLjFLJZrsZbPfPZbsVpfPsb
gvjTzBLztLTjwFjtgLTgtzwdWRqdqRTMSWcWTmWlqdhHHm
ZfzzfmhdpNLNBDDsFfQVCDggfV
HPFjljSnHrqVDgtgQgQMqC
rGnSJHvjSwGzwFhGZG
HqmHRDprrNTZTMbh
CJvzQRQVQCgNzZbzgMNd
vPCvFPcfQFlSJBcfRcPHmDGqWGDqpGtjjtGGHl
wcfpJVHfJBffBBGWRprNRWWWNdhv
DzMzMPqjDnjgCMZPZjzjCjChGdvvbhNdSvrhNWSNqWRRdS
jjCTtnMTDsMBtLRQwwQlFV
JqGnVqCTpDVCTnNLgmPzdgjcGmRg
HrSBJSHblsJthsBBSBhMsrzmdNRccjLzgcLmjgPPjlLL
HSwSttbswWJrbrSWppvVqvvnQVppQQ
JDCHssRTTwcRJDcnCDzRHsHNPZGBtPzFPSPttZSZGBqPBZ
vWhLmTlfrhFqGWSNQNqF
vpMhhpfvmTfhvbLhhgvmgvvlCCJCCMnDnDCnsjRMVDMDswCC
ZgjdlmlmmlJgHJlbZrSDrnrMrmLLDFprmp
TvqdTtdctvvDrGGSDn
CWPBhtWqPPwcdVwlNJfVVNNbbb
vgmrrwlPPrwPBPtmvFcMMrsMSJHscJcMSHDH
TWdLnZjCLGLMQLHBLS
jqVTTZqjdTVjNFNPqBvgvBNl
jmcgMzsmjmfvJwFpFfRWZRWp
drdSldTmCmTDCNCtbRRWqRwtttFZpZWZqw
NLCVLNLTbbTrQNQDvnzschgghnmnHQcH
sRVhVQDVDQRRMQhsqtRRNzqzbNzRqNGp
WdjCLHLjdFnCCnjnFnLHHmPmNJbztJJpprBpbGzbpJbqGWtB
CCFFCnFjdnjCTHmCLTLLCnFnQVhQQVDMhQQVgZZNVVDDsTVh
cGLzZgfzcNNzzRZvjvRmVDmmqCCDSdVVChVnDf
QstsPlWHQlWhMMtpsbtpQtlpDqBVqPSCSVnTTqmVdDVDVdBV
stptQFJrHptlQsFJMHtHhFbvNwvjLGvvNgzgjcwczzJcNJ
SQHCrCFPJZcnWrqn
vfJvJjfGGDggqZGcWD
pjLpRwzhRFtHdMQJ
HNSHNDvnvdffDNfqdZfdStcFFMGmmrRBcmFcrMrqrWFB
VblwzwhwPLlCGGzgzhmFjbjFrMFrjFBmbcrM
hlVPTCCQCCzVlPzhGgPVJCpHtdtSpQSZNNfnZdSSdnSD
cBVmfwqwmWggTRmQzTQl
CDnnHjSDPLCSCLHLHCHNCDFgwJljFQRRhlglbzJFQQhl
GptHLtNGHtCHSnCMtGWdcsqqWwMcqvdsVwfd
HsMFNRNWnbWfZLzWzQ
PqrpjqNdjjhPcdbpvmfzfbffbzmv
GjhhcPjPccVrqcPCldCjssHNnTnNttRwwVMTHMnV
jjCcBswcfnwgpPFPwFFGSFSwFb
VvmVhVvRRQqttRtQDLzLhqRlrBFMWrGPBSMSZSMFqBWZZP
mvQmLRvJtBJVnTcJCjsjsdgp
qgqvPbdMDMMPwpbLFGwtNlNF
TTQmdJTnSllFGtJNtw
CSSHmQHfmVcHjQmSvPBdBDDWMVhMhRMB
WCvQNZdhCDnnPfQPfTzjHcppsHjpsSjHNS
mFMgMBlMmBqHjjBfTjHzBc
grJbrqfqMVFJrlJrtCtvhPPQdnvnnvnwdW
FdQQJRdfSSfrJsRZfFsRSvtDBmDHGtGqbgvnmbDnvDGq
lcMzjCPTVtMqgWWGqn
VpLjcVPczhzznPLcPhTwFQFRZNfFNrNZFpZNsZJR
VWgJhmdDdJDdVPggPSTSTWvvfRzfFfFbNb
jCQtnpGQrHMctnpzRbFfgSwHvgwfwv
ntcMcqLMcQccQLgjBPLdhZDVlJPVdDLJ
RnPnwtqHnJthjLMcWWncMn
msdCrCdNpBBsCrlNTpNBDNGzcLchQjFQzccQLQpzLzWtFS
sTbdTBNrTCTTBCNBlVbwgVPJHtPgPvqgff
QmBsmpmcZQNqPqVnPFVpGh
gDDMDLMJgHfJwJMzfTfdGLhChPtvnGRPRRLFGPGv
DTlzgwfDrrrMWlncbscNnlSW
tBwvGHFttrFrvRgRhCmCmwQmMg
JWbNJZjzfbVjWjBhqfmSnhqCqgnQ
ZZJJJbclzJcsTPdvTTPTBFtHDF
LszmFTFpTmszLrpqSmFpzcvQjtQjvLJgJtcBjgtJjj
VHNwwNCVCChddfwHlWdnlnGRQPcQjRvMWBJJtMMWcvPJMM
nGHNVHhnfnHDNhCfdhNNlwHvmpDrZDmpzmbZSZFsmmbqrrsz";
