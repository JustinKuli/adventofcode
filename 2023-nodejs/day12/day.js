#!/usr/bin/env node

const fs = require('node:fs');

fs.readFile('./sample.txt', (err, data) => {
// fs.readFile('./input.txt', (err, data) => {
    if (err) {
        console.error(err);
    }
    
    const lines = data.toString().split('\n')

    let ways = 0

    lines.forEach((line, i) => {
        const linedata = line.split(" ")
        const rawgroups = linedata[1] 
        const conts = linedata[0].split(".").filter((x) => x.length >= 1)

        debug("main: line", i+1)
        debug("main: rawgroups", rawgroups)

        let ansgroups = {}
        ansgroups[""] = 1

        conts.forEach((cont) => {
            console.log("main: cont", cont)
            let newgroups = {}

            for (const [strippedraw, duplicity] of Object.entries(ansgroups)) {
                let stripped = strippedraw.replaceAll(";", ",")
                stripped = stripped.replace(/,+/, ",").replace(/^,/, "")
                let arrs = {}
                if (stripped) {
                    let remainder = rawgroups.replace(stripped, "")
                    if (remainder.startsWith(",")) {
                        remainder = remainder.replace(",", "")
                        arrs = countArrangements(cont, remainder, false)
                    } else {
                        // previous conts have completely depleted the groups
                        if (!cont.includes("#")) {
                            arrs[""] = 1
                        }
                    }
                } else {
                    arrs = countArrangements(cont, rawgroups, false)
                }

                if (arrs) {
                    Object.keys(arrs).forEach((k) => {
                        let thekey = strippedraw ? strippedraw + ";" + k : k
                        // if (thekey.endsWith(";")) {
                        //     thekey = thekey.replace(/;$/, "")
                        // }
                        newgroups[thekey] = (newgroups[thekey] || 0) + arrs[k] * duplicity
                    })
                }
                
                if (!cont.includes("#")) {
                    // completely optional contgroup has the option of not removing anything
                    newgroups[strippedraw+";"] = (newgroups[strippedraw+";"] || 0) + duplicity
                }
            }

            ansgroups = newgroups
            console.log(ansgroups)
        }) // end conts.forEach

        let lineans = 0
        Object.keys(ansgroups).forEach((k) => {
            let key = k.replaceAll(";", ",").replace(/,+/, ",").replace(/^,/, "").replace(/,$/, "")
            if (key === rawgroups) {
                lineans += ansgroups[k]
            }
        })

        if (lineans === 0) {
            console.log("line", i+1, "ERROR")
        } else {
            console.log("line", i+1, lineans)
        }
        ways += ansgroups[rawgroups]
    }) // end lines.forEach

    console.log("Part One", ways) // 8263 is too high
});

// objects with keys of remaining groups to the number of ways that remainder can be built
function countArrangements(cont, groups, deeper) {
    debug("countArrangements", cont, groups, deeper)

    if (cont === "" || groups === "") {
        debug("returning (0)")
        return deeper ? {} : null
    }

    const grouplengths = groups.split(",").map((n) => Number(n))
    let ans = {}

    if (!cont.includes("?")) { // no unknowns
        if (grouplengths[0] !== cont.length) {
            debug("returning (1)", ans)
            return deeper ? {} : null
        } else {
            const removed = grouplengths[0] + ""
            ans[removed] = 1
            debug("returning (2)", ans)
            return ans
        }
    }

    if (grouplengths[0] > cont.length) {
        debug("returning (3)", ans)
        return deeper ? {} : null
    }

    if (grouplengths[0] === cont.length) {
        const removed = grouplengths[0] + ""
        ans[removed] = 1
        debug("returning (4)", ans)
        return ans
    }

    for (let start = 0; start+grouplengths[0] <= cont.length; start++) {
        const thiscont = cont.slice(start)
        const contRem = thiscont.slice(grouplengths[0])

        debug("in start loop", thiscont, contRem)

        if (contRem[0] === "?" || contRem === "") { // we *can* split the cont after this group
            const subCont = contRem.slice(1)
            const subArrs = countArrangements(subCont, grouplengths.slice(1).join(","), true)

            debug("subArrs", subArrs)

            if (!subCont.includes("#")) { // it's all optional
                debug("found trivial ish", grouplengths[0]+"")
                ans[grouplengths[0]+""] = (ans[grouplengths[0]+""] || 0) + 1
            }

            for (const [subGroupRem, duplicity] of Object.entries(subArrs)) {
                const newKey = subGroupRem ? grouplengths[0] + "," + subGroupRem : grouplengths[0] + ""
                debug("found cool one", newKey, duplicity)
                ans[newKey] = (ans[newKey] || 0) + duplicity
            }
        }

        if (cont[start] !== "?"){
            // can't start the group later, because we can't remove the front
            debug("breaking")
            break
        }
    }

    debug("returning (final)", ans)

    if (Object.keys(ans).length !== 0) {
        return ans
    } else {
        return deeper ? {} : null
    }
}

const debugOn = false

function debug(...args) {
    if (debugOn) {
        console.log(...args)
    }
}
