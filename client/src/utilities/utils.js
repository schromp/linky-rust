export function isValidLink(link) {
    var expression = /[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)?/gi;
    var regex = new RegExp(expression);

    if (link.match(regex) || link === "") {
        return true
    } else {
        return false
    }
}

//TODO regex string longer than 16 -> error
export function isValidShortlink(link) {
    const pattern = /^[a-zA-Z0-9_-]*$/
    return pattern.test(link) ? true : false
}

export function hasHttp(link) {
    const pattern = /^(http|https):\/\//
    return pattern.test(link) ? true : false
}