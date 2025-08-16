sequencer cmaj = {
	seq: [ B3 C3 B3 E3 B3 E3 B3 E3 ],
	bpm: const 240.0,
	spacing: const 0.1,
}

osc osc1 = {
	freq: cmaj,
	wave: saw,
}

amp amp = {
	src0: osc1,
	amp0: osc {
		freq: const 0.2,
		wave: saw,
	},

}

output amp
