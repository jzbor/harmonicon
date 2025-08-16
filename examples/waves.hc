osc sin_fast = {
	freq: const 440.0,
	wave: sin,
}

osc sin_slow = {
	freq: const 0.5,
	wave: sin,
}


amp sin_amp = {
	src: sin_fast,
	mult: sin_slow,
}

osc saw_fast = {
	freq: const 440.0,
	wave: saw,
}

osc saw_slow = {
	freq: const 0.5,
	wave: saw,
}

amp saw_amp = {
	src: saw_fast,
	mult: saw_slow,
}

osc sq_fast = {
	freq: const 440.0,
	wave: sq,
}

osc sq_slow = {
	freq: const 0.5,
	wave: sq,
}

amp sq_amp = {
	src: sq_fast,
	mult: sq_slow,
}

osc tri_fast = {
	freq: const 440.0,
	wave: tri,
}

osc tri_slow = {
	freq: const 0.5,
	wave: tri,
}

amp tri_amp = {
	src: tri_fast,
	mult: tri_slow,
}

output tri_amp
