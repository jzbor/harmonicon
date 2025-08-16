seq notes = {
	seq: [ C E G E C G ],
	bpm: const 220.0,
	spacing: const 0.0,
}


osc oscil = {
	freq: notes,
	wave: sin,
}

osc osc1 = oscil

stereo out = {
	left: oscil,
	right: oscil,
	shift: osc { freq: const 0.1, },
}

amp a = {
	src: oscil,
}


stereo b = a

output out
