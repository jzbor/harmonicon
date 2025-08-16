seq notes = {
	seq: [ G A G A C C ],
	bpm: const 160.0,
	spacing: const 0.0,
}


osc osc = {
	freq: notes,
	wave: saw,
}

osc osc2 = {
	freq: notes,
	wave: sin,
}

osc slow = {
	freq: const 0.5,
	wave: square,
}

amp mixer = {
	src0: osc,
	amp0: slow,
	src1: osc2,
}

output mixer
