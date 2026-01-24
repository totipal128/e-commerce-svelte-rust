export function formatCurrencyIDR(
    value: bigint,
    locales = "id-ID"
) {
    const formatted = value
        .toString()
        .replace(/\B(?=(\d{3})+(?!\d))/g, ".");

    return `Rp ${formatted}`;
}
