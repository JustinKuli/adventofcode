#!/usr/bin/env node

const fs = require('node:fs');

// fs.readFile('./sample.txt', (err, data) => {
fs.readFile('./input.txt', (err, data) => {
    if (err) {
        console.error(err);
    }
    
    const lines = data.toString().split('\n')
    let workflows = {}
    let partsMode = false
    let parts = []
    let p2WFs = {}

    lines.forEach((line) => {
        if (line.length === 0) {
            partsMode = true
            return
        }

        if (!partsMode) {
            let split1 = line.split("{")
            let name = split1[0]
            let allRuleStr = split1[1].slice(0, -1)

            let rules = []
            let p2rules = []
            allRuleStr.split(",").forEach((rule) => {
                p2rules.push(rule) // interpret them later
                if (rule.includes(":")) { // mapping rule
                    let split2 = rule.split(":")
                    let dest = split2[1]
                    if (rule.includes("<")) {
                        let split3 = split2[0].split("<")
                        let key = split3[0]
                        let thresh = split3[1]

                        rules.push((p) => {
                            if (p[key] < thresh) {
                                return dest
                            }
                            return ""
                        })
                    } else if (rule.includes(">")) {
                        let split3 = split2[0].split(">")
                        let key = split3[0]
                        let thresh = split3[1]

                        rules.push((p) => {
                            if (p[key] > thresh) {
                                return dest
                            }
                            return ""
                        })
                    } else {
                        console.log("ERROR")
                    }
                } else { // unconditional send
                    rules.push(() => rule)
                }
            })

            workflows[name] = rules
            p2WFs[name] = p2rules
        } else {
            let formatted = line.replaceAll("{", "{\"").
                replaceAll("=", "\":").
                replaceAll(",", ",\"")
            parts.push(JSON.parse(formatted))
        }
    })

    let sum = 0
    parts.forEach((p) => {
        let wf = "in"
        while (true) {
            let newWF = ""
            workflows[wf].forEach((rule) => {
                if (newWF === "") {
                    let res = rule(p)
                    if (res !== "") {
                        newWF = res
                    }
                }
            })

            if (newWF === "R") {
                return
            } else if (newWF === "A") {
                sum += p.x + p.m + p.a + p.s
                return
            } else {
                wf = newWF
            }
        }
    })

    console.log("Part One", sum)

    let acceptedParts = []

    p2WFs["R"] = ["R"]
    p2WFs["A"] = ["A"]

    let p2parts = [{
        wf: "in",
        xMin: 1, xMax: 4000,
        mMin: 1, mMax: 4000,
        aMin: 1, aMax: 4000,
        sMin: 1, sMax: 4000,
    }]
    while (p2parts.length !== 0) {
        let pt = p2parts.pop()
        let skip = false
        let wfs = pt.wf.split(",")
        let theWF = p2WFs[wfs[wfs.length-1]]
        theWF.forEach((p2rule) => {
            if (skip || (p2rule === "R")) {
                // console.log("Rejected", JSON.stringify(pt))
                return
            } else if (p2rule === "A") {
                // console.log("Accepted", JSON.stringify(pt))
                acceptedParts.push(pt)
                return
            }
            // console.log(JSON.stringify(pt))

            if (!p2rule.includes("<") && !p2rule.includes(">")) { // move all of pt to the specified workflow
                pt.wf = p2rule
                p2parts.push(pt)
                skip = true
                return
            }

            let rulesplit = p2rule.split(":")
            let dest = rulesplit[1]
            let therule = rulesplit[0]

            let key = ""
            let min = -1
            let max = -2
            if (therule.includes("x")) {
                key = "x"
                min = pt.xMin
                max = pt.xMax
            } else if (therule.includes("m")) {
                key = "m"
                min = pt.mMin
                max = pt.mMax
            } else if (therule.includes("a")) {
                key = "a"
                min = pt.aMin
                max = pt.aMax
            } else if (therule.includes("s")) {
                key = "s"
                min = pt.sMin
                max = pt.sMax
            }

            // Update pt to be the bit not affected by the rule.
            // Add the part that is affected to p2parts.
            if (therule.includes("<")) {
                let thresh = Number(therule.split("<")[1])

                if (max < thresh) { // it all moves
                    pt.wf += "," + dest
                    p2parts.push(pt)
                    skip = true
                    return
                }

                if (min < thresh) { // split it
                    let newPt = {...pt}
                    newPt.wf += "," + dest

                    if (key === "x") {
                        pt.xMin = thresh
                        newPt.xMax = thresh - 1
                        p2parts.push(newPt)
                        return
                    } else if (key === "m") {
                        pt.mMin = thresh
                        newPt.mMax = thresh - 1
                        p2parts.push(newPt)
                        return
                    } else if (key === "a") {
                        pt.aMin = thresh
                        newPt.aMax = thresh - 1
                        p2parts.push(newPt)
                        return
                    } else {
                        pt.sMin = thresh
                        newPt.sMax = thresh - 1
                        p2parts.push(newPt)
                        return
                    }
                } // else, none of it moves, leave pt as it is
            } else if (therule.includes(">")) {
                let thresh = Number(therule.split(">")[1])

                if (min > thresh) { // it all moves
                    pt.wf += "," + dest
                    p2parts.push(pt)
                    skip = true
                    return
                }

                if (max > thresh) { // split it
                    let newPt = {...pt}
                    newPt.wf += "," + dest

                    if (key === "x") {
                        pt.xMax = thresh
                        newPt.xMin = thresh + 1
                        p2parts.push(newPt)
                        return
                    } else if (key === "m") {
                        pt.mMax = thresh
                        newPt.mMin = thresh + 1
                        p2parts.push(newPt)
                        return
                    } else if (key === "a") {
                        pt.aMax = thresh
                        newPt.aMin = thresh + 1
                        p2parts.push(newPt)
                        return
                    } else {
                        pt.sMax = thresh
                        newPt.sMin = thresh + 1
                        p2parts.push(newPt)
                        return
                    } 
                } // else it all stays
            }
        })
    }

    let p2ans = 0

    acceptedParts.forEach((ap) => {
        let contribution = ap.xMax - ap.xMin + 1
        contribution *= (ap.mMax - ap.mMin + 1)
        contribution *= (ap.aMax - ap.aMin + 1)
        contribution *= (ap.sMax - ap.sMin + 1)
        
        p2ans += contribution
        // console.log("x", ap.xMin, ap.xMax,
        //     "m", ap.mMin, ap.mMax,
        //     "a", ap.aMin, ap.aMax,
        //     "s", ap.sMin, ap.sMax)
    })

    console.log("Part Two", p2ans)
});
