BitPAl: a bit-parallel, general integer-scoring sequence alignment algorithm, based on https://academic.oup.com/bioinformatics/article/30/22/3166/2390221

I made a rust implementation of this algorithm that takes fasta files as input. 
It's specifically hardcoded with Match score of 1, Mismatch = -1 and a gap = -3
