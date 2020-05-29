"use strict";
import( "../pkg/index.js" ).then(wasm => game(wasm)).catch(console.error);

async function a_perform_benchmark(wasm) {
    perform_benchmark(wasm)
}

function perform_benchmark(wasm) {
    console.log("start benchmark")
    let iterations = 1000;

    const g = wasm.game()
    let t0 = performance.now();
    for (let i = 0; i < iterations; i++) {
        g.tick()
    }
    const time_wasm = performance.now() - t0;
    const state_wasm = hashFnv32a(g.prettier_state(), true)

    const g2 = wasm.game()
    t0 = performance.now();
    g2.multi_tick(iterations)
    const time_wasm_multi = performance.now() - t0;
    const state_wasm_multi = hashFnv32a(g2.prettier_state(), true)

    const gjs = init_game(16)
    t0 = performance.now();
    for (let i = 0; i < iterations; i++) {
        gjs.tick()
    }
    const time_js = performance.now() - t0;
    const state_js = hashFnv32a(gjs.prettier_state(), true)

    const results = document.getElementById("results")
    results.textContent = `
    ${iterations} iterations: JS took ${time_js}ms, WASM took ${time_wasm}ms, Mutli ${time_wasm_multi}ms
    hash of last state is the same: ${state_js === state_wasm} ${state_js === state_wasm_multi}
    `
}

// taken from https://stackoverflow.com/a/22429679
function hashFnv32a(str, asString, seed) {
    /*jshint bitwise:false */
    var i, l,
        hval = (seed === undefined) ? 0x811c9dc5 : seed;

    for (i = 0, l = str.length; i < l; i++) {
        hval ^= str.charCodeAt(i);
        hval += (hval << 1) + (hval << 4) + (hval << 7) + (hval << 8) + (hval << 24);
    }
    if (asString) {
        // Convert to 8 digit hex string
        return ("0000000" + (hval >>> 0).toString(16)).substr(-8);
    }
    return hval >>> 0;
}

function game(wasm) {
    const bench_button = document.getElementById("run_benchmark")
    bench_button.addEventListener("click", () => {
        bench_button.disable = true;
        a_perform_benchmark(wasm).then(() => bench_button.disable = false);
    }, false)
    perform_benchmark(wasm)

    const gameElement = document.getElementById("game")
    // const g = wasm.game()
    const g = init_game(16)
    const renderloop = async () => {
        gameElement.textContent = g.prettier_state()
        g.tick()
        await new Promise(r => setTimeout(r, 100));
        requestAnimationFrame(renderloop)
    }
    requestAnimationFrame(renderloop)
}

class Life {
    constructor(size) {
        this.size = size
        this.universe = init_array(size)
        this.last = init_array(size)
        this.ticks = 0
    }

    set_alive(x, y) {
        if (x < 0 || x >= this.size || y < 0 || y >= this.size) return
        this.universe[x][y] = 1
    }

    will_be_alive(last, x, y) {
        let alive_neighbors = 0;
        for (let i = -1; i <= 1; i++) {
            for (let j = -1; j <= 1; j++) {
                if (i === 0 && j === 0) { // don' count self
                    continue;
                }
                // turns into a wrapping universe
                let x_i = (x + i);
                if (x_i < 0) x_i += this.size
                if (x_i >= this.size) x_i -= this.size
                let y_j = (y + j);
                if (y_j < 0) y_j += this.size
                if (y_j >= this.size) y_j -= this.size
                alive_neighbors += last[x_i][y_j];
            }
        }
        if (last[x][y] === 0 && alive_neighbors === 3) {
            return 1
        } else if (last[x][y] === 1 && (alive_neighbors === 2 || alive_neighbors === 3)) {
            return 1
        } else {
            return 0
        }
    }

    tick() {
        this.ticks++
        let tmp = this.last
        this.last = this.universe
        this.universe = tmp


        for (let x = 0; x < this.size; x++) {
            for (let y = 0; y < this.size; y++) {
                this.universe[x][y] = this.will_be_alive(this.last, x, y)
            }
        }
    }

    prettier_state() {
        let s = "";
        for (let x = 0; x < this.universe.length; x++) {
            for (let y = 0; y < this.universe[x].length; y++) {
                s += (this.universe[x][y] === 1) ? "█" : "░"
            }
            s += "\n"
        }

        return s
    }
}

function init_game() {
    let life = new Life(16)
    life.set_alive(1, 2);
    life.set_alive(2, 3);
    life.set_alive(3, 1);
    life.set_alive(3, 2);
    life.set_alive(3, 3);
    return life
}

function init_array(size) {
    let u = []
    for (let i = 0; i < size; i++) {
        u.push([])
        for (let j = 0; j < size; j++) {
            u[i].push(0)
        }
    }
    return u
}
