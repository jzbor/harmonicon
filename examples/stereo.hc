osc slow = {
	freq: const 0.5,
}

osc fast = {
	freq: const 440.0,
}

stereo st = {
	left: fast,
	right: fast,
	shift: slow,
}

output st
