export interface WeatherCity {
    id: string
    label: string
    /** Value stored in settings; jarvis-core maps this to fixed coordinates */
    value: string
}

export const WEATHER_CITIES: WeatherCity[] = [
    { id: "moscow", label: "Москва", value: "Москва" },
    { id: "chelyabinsk", label: "Челябинск", value: "Челябинск" },
    { id: "yakutsk", label: "Якутск", value: "Якутск" },
]

const DEFAULT_CITY = WEATHER_CITIES[0].value

export function resolveWeatherCity(raw: string | null | undefined): string {
    const trimmed = (raw ?? "").trim()
    if (!trimmed) return DEFAULT_CITY

    const lower = trimmed.toLowerCase()
    for (const city of WEATHER_CITIES) {
        if (
            city.value.toLowerCase() === lower ||
            city.label.toLowerCase() === lower ||
            city.id === lower
        ) {
            return city.value
        }
    }

    return DEFAULT_CITY
}

export function weatherCityLabel(value: string): string {
    const city = WEATHER_CITIES.find((c) => c.value === value)
    return city?.label ?? value
}
