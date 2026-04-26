export function formatCurrencyIDR(
    value: any,
    locales = "id-ID"
) {
    if (value === null || value === undefined || value === "") return "Rp 0";
    
    // Ensure it's a number and remove formatting if it's a string
    const num = Number(value.toString().replace(/\./g, ""));
    if (isNaN(num)) return "Rp 0";

    const formatted = Math.floor(num)
        .toString()
        .replace(/\B(?=(\d{3})+(?!\d))/g, ".");

    return `Rp ${formatted}`;
}
