export function FormatCurrency(
    data: bigint,
    currency: "IDR",
    locales = "id-ID"
) {
    return Number(data.toString().replace(/\./g, " ")).toLocaleString(locales, {
        style: 'currency',
        currency: currency
    })
}
