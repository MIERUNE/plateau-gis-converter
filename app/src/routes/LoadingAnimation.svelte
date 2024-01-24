<div class="loader flex flex-col gap-3">
	<p class="text-white font-semibold text-2xl">変換中 ...</p>

	<div class="loader ball-pulse flex">
		<div />
		<div />
		<div />
	</div>
</div>

<style>
	@keyframes scale {
		0% {
			transform: scale(1);
			opacity: 1;
		}
		45% {
			transform: scale(0);
			opacity: 0.7;
		}
		80% {
			transform: scale(1);
			opacity: 1;
		}
	}

	.ball-pulse > div:nth-child(1) {
		animation: scale 1s -0.24s infinite cubic-bezier(0.2, 0.68, 0.18, 1.08);
	}

	.ball-pulse > div:nth-child(2) {
		animation: scale 1s -0.12s infinite cubic-bezier(0.2, 0.68, 0.18, 1.08);
	}

	.ball-pulse > div:nth-child(3) {
		animation: scale 1s 0s infinite cubic-bezier(0.2, 0.68, 0.18, 1.08);
	}

	.ball-pulse > div {
		@apply bg-white;
		@apply w-4 h-4 rounded-full;
		@apply m-2;
		animation-fill-mode: both;
	}
</style>
