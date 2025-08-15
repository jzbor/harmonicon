const i = 220.0

osc slow = {
	freq: const 0.5,
}

osc fast = {
	freq: const 8.0,
}

osc osc1 = {
	freq: amp { src: fast, mult: const 440.0, },
}

amp amp1 = {
	src: osc1,
	mult: slow,
}

amp out = {
	src: amp1,
	mult: const 0.5,
}
