const i = 220.0

osc slow = {
	freq: const 0.1,
}

osc freq = {
	freq: const 80.0,
}

stereo s = {
	left: freq,
	right: freq,
	shift: slow,
}

amp out = {
	src: s,
	mult: const 0.5,
}
