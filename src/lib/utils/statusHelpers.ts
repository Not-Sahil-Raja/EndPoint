export function getStatusDisplay(status: string): { text: string; class: string } {
    if (!status || status === "unknown") {
        return { text: "Unknown", class: "text-gray-600 font-semibold" };
    }
    const code = parseInt(status.split(" ")[0], 10);
    if (isNaN(code)) {
        return { text: "Unknown", class: "text-gray-600 font-semibold" };
    }
    if (code >= 200 && code < 400) {
        return { text: code.toString(), class: "text-green-600 font-semibold" };
    }
    return { text: code.toString(), class: "text-red-600 font-semibold" };
}