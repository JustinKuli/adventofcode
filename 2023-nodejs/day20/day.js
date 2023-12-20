#!/usr/bin/env node

const fs = require('node:fs');

let memos = {}

// fs.readFile('./sample2.txt', (err, data) => {
fs.readFile('./input.txt', (err, data) => {
    if (err) {
        console.error(err);
    }
    
    const lines = data.toString().split('\n')

    let nodes = {} // name to type, dests, state?, high?
    // `&` nodes have .state: an obj from each input to its last level
    // `%` nodes have a boolean at .off

    lines.forEach((line) => {
        let parts = line.split(" -> ")
        let namepart = parts[0]
        let destpart = parts[1]

        let node = {}

        let name = ""
        if (namepart.startsWith("%")) {
            name = namepart.slice(1)
            node.type = "%"
            node.off = true
        } else if (namepart.startsWith("&")) {
            name = namepart.slice(1)
            node.type = "&"
            node.state = {}
        } else {
            name = "broadcaster"
            node.type = "broadcaster"
        }

        node.dests = destpart.split(", ")

        nodes[name] = node
    })

    // second pass to ensure all conjunction nodes are known for their inputs to be recorded
    lines.forEach((line) => {
        let parts = line.split(" -> ")
        let name = parts[0]
        let dests = parts[1].split(", ")

        if (name[0] === "&" || name[0] === "%") {
            name = name.slice(1)
        }

        dests.forEach((d) => {
            if ((d in nodes) && (nodes[d].type === "&")) {
                nodes[d].state[name] = "low"
            }
        })
    })

    let initialNodes = JSON.stringify(nodes)

    let loopHighs = 0
    let loopLows = 0
    let loopLen = 0
    while (true && loopLen < 1000) {
        let ans = pushButton(nodes)
        if (ans.loopFound) {
            break
        }
        loopLen++
        loopHighs += ans.highs
        loopLows += ans.lows
    }

    if (loopLen === 1000) {
        console.log("Part One", loopHighs, loopLows, loopHighs*loopLows)
    } else {
        let factor = 1000 / loopLen
        let rem = 1000 % loopLen
        console.log("remainder", rem)

        let totalHighs = loopHighs * factor
        let totalLows = loopLows * factor

        console.log("Part One with loop", totalHighs * totalLows)
    }

    nodes = JSON.parse(initialNodes) // reset

    // let rxLow = false
    // let count = 0
    // while (!rxLow) {
    //     let ans = pushButton(nodes)
    //     count++
    //     rxLow = ans.rxLow
    //     if (count % 1000 === 0) {
    //         console.log(count)
    //     }
    // }

    // console.log("Part Two", count)
});

function pushButton(nodes) {
    let key = JSON.stringify(nodes)
    let rxLow = false
    if (key in memos) {
        return {loopFound: true, ...memos[key]}
    }

    let queue = [] // use push and shift
    let lows = 0
    let highs = 0

    lows++ // initial button press
    nodes["broadcaster"].dests.forEach((d) => {
        queue.push({dest: d, level: "low", origin: "broadcaster"})
        // console.log("sent", queue[queue.length-1])
        lows++
    })

    while (queue.length !== 0) {
        let pulse = queue.shift()

        // console.log("queue", queue)
        // console.log("handling", pulse)
        // console.log("nodes", nodes)

        if (!(pulse.dest in nodes)) {
            continue
        }

        let node = nodes[pulse.dest]

        if (node.type === "%") {
            if (pulse.level === "low") {
                node.off = !node.off

                node.dests.forEach((d) => {
                    if (node.off) {
                        queue.push({dest: d, level: "low", origin: pulse.dest})
                        // console.log("sent", queue[queue.length-1])
                        lows++
                    } else {
                        queue.push({dest: d, level: "high", origin: pulse.dest})
                        // console.log("sent", queue[queue.length-1])
                        highs++
                    }
                })
            }
        } else if (node.type === "&") {
            node.state[pulse.origin] = pulse.level

            let allHigh = true
            Object.values(node.state).forEach((v) => {
                if (v === "low") {
                    allHigh = false
                }
            })

            node.dests.forEach((d) => {
                if (allHigh) {
                    queue.push({dest: d, level: "low", origin: pulse.dest})
                    // console.log("sent", queue[queue.length-1])
                    lows++
                } else {
                    queue.push({dest: d, level: "high", origin: pulse.dest})
                    // console.log("sent", queue[queue.length-1])
                    highs++
                }
            })
        }
    }

    printNodeState(nodes)

    memos[key] = {highs: highs, lows: lows, rxLow: rxLow}
    return {loopFound: false, ...memos[key]}
}

let labelPrinted = false

function printNodeState(nodes) {
    let cons = []
    let flops = []

    Object.keys(nodes).forEach((k) => {
        if (nodes[k].type === "%") {
            flops.push(k)
        } else if (nodes[k].type === "&") {
            cons.push(k)
        }
    })

    let labelLine = ""
    let valueLine = ""

    // cons.forEach((c) => {
    //     labelLine += " &" + c + " "
    //     let total = Object.keys(nodes[c].state).length
    //     let highs = Object.values(nodes[c].state).filter((x) => x === "high").length 
    //     valueLine += " " + highs + "/" + total + " "
    // })

    flops.forEach((f) => {
        labelLine += "%" + f + " "
        valueLine += "  " + (nodes[f].off ? "." : "O") + " "
    })

    if (!labelPrinted) {
        console.log(labelLine)
        labelPrinted = true
    }
    console.log(valueLine)
}
