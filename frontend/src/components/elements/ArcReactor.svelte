<script lang="ts">
    import { onDestroy } from "svelte"
    import { jarvisState } from "@/stores"

    export let activating = false
    export let online = false

    const BOOT_MS = 3000
    const TASK_RAMP_MS = 2200
    /** ~14% быстрее при задаче: 10s → ~8.8s на оборот */
    const TASK_SPIN_RATE = 1.14
    const TASK_INTENSITY = 0.42

    let boot = 0
    let taskBoost = 0
    let bootRaf = 0
    let taskRaf = 0
    let bootPhase: "idle" | "up" | "down" = "idle"
    let prevTaskBoostTarget = -1

    $: spinRate = 1 + (TASK_SPIN_RATE - 1) * taskBoost

    function easeInOutCubic(t: number) {
        return t < 0.5 ? 4 * t * t * t : 1 - Math.pow(-2 * t + 2, 3) / 2
    }

    function cancelBoot() {
        if (bootRaf) cancelAnimationFrame(bootRaf)
        bootRaf = 0
    }

    function cancelTaskRamp() {
        if (taskRaf) cancelAnimationFrame(taskRaf)
        taskRaf = 0
    }

    function rampTaskBoost(to: number) {
        cancelTaskRamp()
        const from = taskBoost
        const t0 = performance.now()

        const step = (now: number) => {
            const p = Math.min(1, (now - t0) / TASK_RAMP_MS)
            taskBoost = from + (to - from) * easeInOutCubic(p)

            if (p < 1) {
                taskRaf = requestAnimationFrame(step)
            } else {
                taskRaf = 0
                taskBoost = to
            }
        }

        taskRaf = requestAnimationFrame(step)
    }

    function animateBoot(to: number, duration = BOOT_MS) {
        cancelBoot()
        const from = boot
        const t0 = performance.now()

        const step = (now: number) => {
            const p = Math.min(1, (now - t0) / duration)
            boot = from + (to - from) * easeInOutCubic(p)
            if (p < 1) {
                bootRaf = requestAnimationFrame(step)
            } else {
                bootRaf = 0
                bootPhase = "idle"
            }
        }

        bootRaf = requestAnimationFrame(step)
    }

    $: taskActive =
        online &&
        boot >= 0.99 &&
        ($jarvisState === "listening" || $jarvisState === "processing")

    $: taskBoostTarget = taskActive && boot >= 0.99 ? 1 : 0

    $: if (taskBoostTarget !== prevTaskBoostTarget) {
        prevTaskBoostTarget = taskBoostTarget
        rampTaskBoost(taskBoostTarget)
    }

    $: {
        if (activating) {
            if (bootPhase !== "up") {
                bootPhase = "up"
                animateBoot(1)
            }
        } else if (!online && boot > 0.01) {
            if (bootPhase !== "down") {
                bootPhase = "down"
                animateBoot(0, 1200)
            }
        } else if (online && boot < 0.995) {
            if (bootPhase !== "up") {
                bootPhase = "up"
                animateBoot(1, 600)
            }
        } else if (!activating && !online) {
            bootPhase = "idle"
        }
    }

    onDestroy(() => {
        cancelBoot()
        cancelTaskRamp()
    })
</script>

<div
    id="arc-reactor"
    class="reactor-container booting arc-white"
    class:command-active={taskBoost > 0.02}
    style="--boot: {boot}; --intensity: {TASK_INTENSITY * taskBoost}; --spin-rate: {spinRate}"
>
    <div class="reactor-container-inner circle abs-center">
        <ul class="marks">
            {#each Array(60) as _, i}
                <li></li>
            {/each}
        </ul>
        <div class="e7">
            <div class="semi_arc_3 e5_1">
                <div class="semi_arc_3 e5_1_ghost"></div>
                <div class="semi_arc_3 e5_2">
                    <div class="semi_arc_3 e5_2_ghost"></div>
                    <div class="semi_arc_3 e5_3">
                        <div class="semi_arc_3 e5_3_ghost"></div>
                        <div class="semi_arc_3 e5_4">
                            <div class="semi_arc_3 e5_4_ghost"></div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
    <div class="tunnel circle abs-center"></div>
    <div class="core-wrapper circle abs-center"></div>
    <div class="core-outer circle abs-center"></div>
    <div class="core-inner circle abs-center"></div>
    <div class="coil-container">
        {#each Array(8) as _, i}
            <div class="coil coil-{i + 1}"></div>
        {/each}
    </div>
</div>

<style lang="scss" global>
    // [ ARC REACTOR VARIABLES ]
    $arc-radius: 133px;
    $arc-container-size: 195%;
    $arc-spacing: 93%;

    // arc thickness per level
    $arc-thickness-1: 1px;
    $arc-thickness-2: 2px;
    $arc-thickness-3: 3px;
    $arc-thickness-4: 4px;

    // marks (lines) settings
    $mark-width: 2.5px;
    $mark-height: 11px;

    // [ DEFAULT THEME - CYAN ]
    .reactor-container {
        --arc-color: 2, 254, 255;           // RGB values for easy rgba()
        --arc-glow: #52fefe;
        --arc-glow-rgb: 82, 254, 254;
        --arc-core-border: #1b4e5f;
        --arc-core-bg: #073c4b;

        width: 300px;
        height: 300px;
        margin: auto;
        position: relative;
        border-radius: 50%;
        transition: transform 1.2s ease, opacity 1.2s ease, filter 1.2s ease;
        transform: scale(0.95);
        opacity: 0.9;

        ul {
            list-style: none;
            margin: 0;
            padding: 0;
        }
    }

    // [ COLOR THEMES ]
    .reactor-container.arc-cyan {
        --arc-color: 2, 254, 255;
        --arc-glow: #52fefe;
        --arc-glow-rgb: 82, 254, 254;
        --arc-core-border: #1b4e5f;
        --arc-core-bg: #073c4b;
    }

    .reactor-container.arc-red {
        --arc-color: 255, 50, 50;
        --arc-glow: #ff5050;
        --arc-glow-rgb: 255, 80, 80;
        --arc-core-border: #5f1b1b;
        --arc-core-bg: #4b0707;
    }

    .reactor-container.arc-orange {
        --arc-color: 255, 150, 50;
        --arc-glow: #ff9632;
        --arc-glow-rgb: 255, 150, 50;
        --arc-core-border: #5f3c1b;
        --arc-core-bg: #4b2a07;
    }

    .reactor-container.arc-yellow {
        --arc-color: 255, 230, 50;
        --arc-glow: #ffe632;
        --arc-glow-rgb: 255, 230, 50;
        --arc-core-border: #5f5a1b;
        --arc-core-bg: #4b4507;
    }

    .reactor-container.arc-green {
        --arc-color: 50, 255, 100;
        --arc-glow: #32ff64;
        --arc-glow-rgb: 50, 255, 100;
        --arc-core-border: #1b5f2a;
        --arc-core-bg: #074b15;
    }

    .reactor-container.arc-blue {
        --arc-color: 50, 150, 255;
        --arc-glow: #3296ff;
        --arc-glow-rgb: 50, 150, 255;
        --arc-core-border: #1b3c5f;
        --arc-core-bg: #072a4b;
    }

    .reactor-container.arc-purple {
        --arc-color: 180, 100, 255;
        --arc-glow: #b464ff;
        --arc-glow-rgb: 180, 100, 255;
        --arc-core-border: #3c1b5f;
        --arc-core-bg: #28074b;
    }

    .reactor-container.arc-pink {
        --arc-color: 255, 100, 200;
        --arc-glow: #ff64c8;
        --arc-glow-rgb: 255, 100, 200;
        --arc-core-border: #5f1b4a;
        --arc-core-bg: #4b0735;
    }

    .reactor-container.arc-white {
        --arc-color: 255, 255, 255;
        --arc-glow: #ffffff;
        --arc-glow-rgb: 255, 255, 255;
        --arc-core-border: #4a4a4a;
        --arc-core-bg: #2a2a2a;
    }

    // [ BACKGROUND GLOW ]
    .reactor-container::before {
        content: '';
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        width: 1000px;
        height: 1000px;
        border-radius: 50%;
        background: radial-gradient(
            circle,
            rgba(var(--arc-color), 0.20) 0%,
            rgba(var(--arc-color), 0.15) 30%,
            rgba(var(--arc-color), 0.10) 40%,
            rgba(var(--arc-color), 0.04) 50%,
            transparent 70%
        );
        z-index: -1;
        pointer-events: none;
        transition: opacity 0.5s ease, transform 0.5s ease;
    }

    // [ CORE ELEMENTS - using CSS vars ]
    .reactor-container-inner {
        height: 238px;
        width: 238px;
        background-color: #161a1b;
        box-shadow: 0px 0px 50px 15px rgba(var(--arc-color), 0.3), 
                    inset 0px 0px 50px 15px rgba(var(--arc-color), 0.3);
        transition: box-shadow 0.5s ease;
    }

    .circle { border-radius: 50%; }

    .abs-center {
        position: absolute;
        top: 0;
        right: 0;
        bottom: 0;
        left: 0;
        margin: auto;
    }

    .core-inner {
        width: 70px;
        height: 70px;
        border: 5px solid var(--arc-core-border);
        background-color: #ffffff;
        box-shadow: 0px 0px 7px 5px var(--arc-glow), 
                    0px 0px 10px 10px var(--arc-glow) inset;
        transition: box-shadow 0.5s ease, transform 0.5s ease;
    }

    @keyframes core-pulse-idle {
        0% { transform: scale(1); }
        50% { transform: scale(1.03); box-shadow: 0px 0px 10px 6px var(--arc-glow), 0px 0px 12px 12px var(--arc-glow) inset; }
        100% { transform: scale(1); }
    }

    @keyframes core-pulse-active {
        0% { transform: scale(1); box-shadow: 0px 0px 15px 10px var(--arc-glow), 0px 0px 20px 15px var(--arc-glow) inset; }
        50% { transform: scale(1.1); box-shadow: 0px 0px 25px 15px var(--arc-glow), 0px 0px 30px 20px var(--arc-glow) inset; }
        100% { transform: scale(1); box-shadow: 0px 0px 15px 10px var(--arc-glow), 0px 0px 20px 15px var(--arc-glow) inset; }
    }

    .core-outer {
        width: 120px;
        height: 120px;
        border: 1px solid var(--arc-glow);
        background-color: #ffffff;
        box-shadow: 0px 0px 2px 1px var(--arc-glow), 
                    0px 0px 10px 5px var(--arc-glow) inset;
        transition: box-shadow 0.5s ease;
    }

    .core-wrapper {
        width: 180px;
        height: 180px;
        background-color: var(--arc-core-bg);
        box-shadow: 0px 0px 5px 4px var(--arc-glow), 
                    0px 0px 6px 2px var(--arc-glow) inset;
        transition: box-shadow 0.5s ease;
    }

    .tunnel {
        width: 220px;
        height: 220px;
        background-color: #ffffff;
        box-shadow: 0px 0px 5px 1px var(--arc-glow), 
                    0px 0px 5px 4px var(--arc-glow) inset;
        transition: box-shadow 0.5s ease;
    }

    .coil-container {
        position: relative;
        width: 100%;
        height: 100%;
        animation: reactor-anim 10s infinite linear;
    }

    .coil {
        position: absolute;
        width: 30px;
        height: 20px;
        top: calc(50% - 110px);
        left: calc(50% - 15px);
        transform-origin: 15px 110px;
        background-color: var(--arc-core-bg);
        box-shadow: 0px 0px 5px var(--arc-glow) inset;
    }

    @for $i from 1 through 8 {
        .coil-#{$i} {
            transform: rotate(#{($i - 1) * 45}deg);
        }
    }

    @keyframes reactor-anim {
        from { transform: rotate(0deg); }
        to { transform: rotate(360deg); }
    }

    .e7 {
        position: absolute;
        z-index: 1;
        width: $arc-container-size;
        height: $arc-container-size;
        left: 50%;
        top: 50%;
        transform: translate(-50%, -50%);
        border: 6px solid transparent;
        background: transparent;
        border-radius: 50%;
        transition: opacity 0.5s ease;
        text-align: center;
        opacity: 0.5;
    }

    .semi_arc_3 {
        $offset: calc((100% - #{$arc-spacing}) / 2);
        
        position: absolute;
        width: $arc-spacing;
        height: $arc-spacing;
        left: $offset;
        top: $offset;
        border-style: solid;
        border-color: transparent;
        border-radius: 50%;
        box-sizing: border-box;
        animation: rotate 6s linear infinite;
        overflow: hidden;
    }

    // [ MAIN ARCS ]
    .e5_1 {
        border-width: $arc-thickness-1;
        border-top-color: rgba(var(--arc-color), 0.25);
        border-right-color: transparent;
        border-bottom-color: transparent;
        border-left-color: rgba(var(--arc-color), 0.25);
        animation: rotate 8s linear infinite;
    }

    .e5_2 {
        border-width: $arc-thickness-2;
        border-top-color: transparent;
        border-right-color: rgba(var(--arc-color), 0.4);
        border-bottom-color: rgba(var(--arc-color), 0.4);
        border-left-color: transparent;
        animation: rotate_anti 6s linear infinite;
    }

    .e5_3 {
        border-width: $arc-thickness-3;
        border-top-color: rgba(var(--arc-color), 0.6);
        border-right-color: transparent;
        border-bottom-color: transparent;
        border-left-color: transparent;
        animation: rotate 4s linear infinite;
    }

    .e5_4 {
        border-width: $arc-thickness-4;
        border-top-color: transparent;
        border-right-color: transparent;
        border-bottom-color: rgba(var(--arc-color), 0.8);
        border-left-color: transparent;
        animation: rotate_anti 5s linear infinite;
    }

    // [ GHOST ARCS ]
    .e5_1_ghost {
        border-width: $arc-thickness-1;
        border-top-color: rgba(var(--arc-color), 0.08);
        border-right-color: rgba(var(--arc-color), 0.08);
        border-bottom-color: transparent;
        border-left-color: transparent;
        animation: rotate_anti 12s linear infinite;
    }

    .e5_2_ghost {
        border-width: $arc-thickness-2;
        border-top-color: rgba(var(--arc-color), 0.12);
        border-right-color: transparent;
        border-bottom-color: transparent;
        border-left-color: rgba(var(--arc-color), 0.12);
        animation: rotate 9s linear infinite;
    }

    .e5_3_ghost {
        border-width: $arc-thickness-3;
        border-top-color: transparent;
        border-right-color: rgba(var(--arc-color), 0.18);
        border-bottom-color: rgba(var(--arc-color), 0.18);
        border-left-color: transparent;
        animation: rotate_anti 6s linear infinite;
    }

    .e5_4_ghost {
        border-width: $arc-thickness-4;
        border-top-color: rgba(var(--arc-color), 0.24);
        border-right-color: transparent;
        border-bottom-color: transparent;
        border-left-color: rgba(var(--arc-color), 0.24);
        animation: rotate 7.5s linear infinite;
    }

    @keyframes rotate {
        0% { transform: rotateZ(0deg); }
        100% { transform: rotateZ(360deg); }
    }

    @keyframes rotate_anti {
        0% { transform: rotateZ(0deg); }
        100% { transform: rotateZ(-360deg); }
    }

    // [ MARKS ]
    .marks {
        position: absolute;
        width: 100%;
        height: 100%;
        top: 0;
        left: 0;
    }

    .marks li {
        width: $mark-width;
        height: $mark-height;
        background: rgba(var(--arc-color), 0.8);
        position: absolute;
        left: 50%;
        top: 50%;
        margin-left: calc(-#{$mark-width} / 2);
        margin-top: calc(-#{$mark-height} / 2);
        animation: colour_ease2 3s infinite ease-in-out;
        border-radius: 2px;
    }

    @keyframes colour_ease2 {
        0% { background: rgba(var(--arc-color), 1); box-shadow: 0 0 4px rgba(var(--arc-color), 0.8); }
        50% { background: rgba(var(--arc-color), 0.2); box-shadow: none; }
        100% { background: rgba(var(--arc-color), 1); box-shadow: 0 0 4px rgba(var(--arc-color), 0.8); }
    }

    @for $i from 1 through 60 {
        .marks li:nth-child(#{$i}) {
            transform: rotate(#{$i * 6}deg) translateY(-$arc-radius);
            animation-delay: #{$i * 0.05}s;
        }
    }

    @keyframes bg-pulse {
        0%, 100% { 
            opacity: 1;
            transform: translate(-50%, -50%) scale(1.1);
        }
        50% { 
            opacity: 0.85;
            transform: translate(-50%, -50%) scale(1.05);
        }
    }

    // [ BOOT + INTENSITY — boot: вкл.; --spin-rate: чуть быстрее при задаче (rAF, без transition) ]
    .reactor-container.booting {
        --intensity: 0;
        --spin-rate: 1;
        transform: scale(calc(0.85 + 0.1 * var(--boot) + 0.02 * var(--intensity)));
        opacity: calc(0.4 + 0.5 * var(--boot) + 0.04 * var(--intensity));
        filter: grayscale(calc(0.7 * (1 - var(--boot)))) brightness(calc(0.6 + 0.4 * var(--boot)));

        .coil-container {
            animation-duration: calc((35s - 25s * var(--boot)) / var(--spin-rate));
        }

        .e7 {
            opacity: calc(0.3 + 0.3 * var(--boot) + 0.12 * var(--intensity));
        }

        .core-inner {
            box-shadow:
                0 0 calc(7px + 8px * var(--boot) + 3px * var(--intensity)) calc(5px + 5px * var(--boot) + 2px * var(--intensity)) var(--arc-glow),
                0 0 calc(10px + 10px * var(--boot) + 4px * var(--intensity)) calc(10px + 5px * var(--boot) + 2px * var(--intensity)) var(--arc-glow) inset;
            animation: core-pulse-idle calc(6s - 2s * var(--boot)) ease-in-out infinite;
        }

        .core-outer {
            box-shadow:
                0 0 calc(2px + 1px * var(--intensity)) calc(1px + 0.5px * var(--intensity)) var(--arc-glow),
                0 0 calc(10px + 2px * var(--intensity)) calc(5px + 2px * var(--intensity)) var(--arc-glow) inset;
        }

        .core-wrapper {
            box-shadow:
                0 0 calc(5px + 2px * var(--intensity)) calc(4px + 1px * var(--intensity)) var(--arc-glow),
                0 0 calc(6px + 2px * var(--intensity)) calc(2px + 1px * var(--intensity)) var(--arc-glow) inset;
        }

        .tunnel {
            box-shadow:
                0 0 calc(5px + 2px * var(--intensity)) calc(1px + 0.5px * var(--intensity)) var(--arc-glow),
                0 0 calc(5px + 2px * var(--intensity)) calc(4px + 1px * var(--intensity)) var(--arc-glow) inset;
        }

        .reactor-container-inner {
            box-shadow:
                0 0 calc(50px + 20px * var(--boot) + 8px * var(--intensity)) calc(15px + 10px * var(--boot) + 4px * var(--intensity)) rgba(var(--arc-color), calc(0.3 * var(--boot) + 0.06 * var(--intensity))),
                inset 0 0 calc(50px + 20px * var(--boot) + 8px * var(--intensity)) calc(15px + 10px * var(--boot) + 4px * var(--intensity)) rgba(var(--arc-color), calc(0.3 * var(--boot) + 0.06 * var(--intensity)));
        }

        .e5_1 {
            border-top-color: rgba(var(--arc-color), calc(0.1 + 0.1 * var(--boot) + 0.06 * var(--intensity)));
            border-left-color: rgba(var(--arc-color), calc(0.1 + 0.1 * var(--boot) + 0.06 * var(--intensity)));
            animation-duration: calc((35s - 26s * var(--boot)) / var(--spin-rate));
        }

        .e5_2 {
            border-right-color: rgba(var(--arc-color), calc(0.15 + 0.2 * var(--boot) + 0.08 * var(--intensity)));
            border-bottom-color: rgba(var(--arc-color), calc(0.15 + 0.2 * var(--boot) + 0.08 * var(--intensity)));
            animation-duration: calc((30s - 15s * var(--boot)) / var(--spin-rate));
        }

        .e5_3 {
            border-top-color: rgba(var(--arc-color), calc(0.2 + 0.3 * var(--boot) + 0.1 * var(--intensity)));
            animation-duration: calc((25s - 13s * var(--boot)) / var(--spin-rate));
        }

        .e5_4 {
            border-bottom-color: rgba(var(--arc-color), calc(0.25 + 0.4 * var(--boot) + 0.12 * var(--intensity)));
            animation-duration: calc((28s - 14s * var(--boot)) / var(--spin-rate));
        }

        .e5_1_ghost {
            border-top-color: rgba(var(--arc-color), calc(0.03 + 0.03 * var(--boot) + 0.02 * var(--intensity)));
            border-right-color: rgba(var(--arc-color), calc(0.03 + 0.03 * var(--boot) + 0.02 * var(--intensity)));
            animation-duration: calc((50s - 36s * var(--boot)) / var(--spin-rate));
        }

        .e5_2_ghost {
            border-top-color: rgba(var(--arc-color), calc(0.05 + 0.05 * var(--boot) + 0.03 * var(--intensity)));
            border-left-color: rgba(var(--arc-color), calc(0.05 + 0.05 * var(--boot) + 0.03 * var(--intensity)));
            animation-duration: calc((45s - 23s * var(--boot)) / var(--spin-rate));
        }

        .e5_3_ghost {
            border-right-color: rgba(var(--arc-color), calc(0.06 + 0.09 * var(--boot) + 0.04 * var(--intensity)));
            border-bottom-color: rgba(var(--arc-color), calc(0.06 + 0.09 * var(--boot) + 0.04 * var(--intensity)));
            animation-duration: calc((40s - 22s * var(--boot)) / var(--spin-rate));
        }

        .e5_4_ghost {
            border-top-color: rgba(var(--arc-color), calc(0.08 + 0.12 * var(--boot) + 0.05 * var(--intensity)));
            border-left-color: rgba(var(--arc-color), calc(0.08 + 0.12 * var(--boot) + 0.05 * var(--intensity)));
            animation-duration: calc((42s - 21s * var(--boot)) / var(--spin-rate));
        }

        .marks li {
            background: rgba(var(--arc-color), calc(0.3 + 0.7 * var(--boot)));
            animation-duration: 3s;
        }
    }

    .reactor-container.booting::before {
        opacity: calc(0.3 + 0.4 * var(--boot) + 0.1 * var(--intensity));
        transform: translate(-50%, -50%) scale(calc(0.7 + 0.2 * var(--boot) + 0.06 * var(--intensity)));
    }

    .reactor-container.booting.command-active::before {
        animation: bg-pulse 5s ease-in-out infinite;
    }

    .reactor-container.booting .coil-container,
    .reactor-container.booting .e5_1,
    .reactor-container.booting .e5_2,
    .reactor-container.booting .e5_3,
    .reactor-container.booting .e5_4,
    .reactor-container.booting .e5_1_ghost,
    .reactor-container.booting .e5_2_ghost,
    .reactor-container.booting .e5_3_ghost,
    .reactor-container.booting .e5_4_ghost {
        animation-timing-function: linear;
    }
</style>