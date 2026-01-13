import {format, parseISO} from 'date-fns';

export function FormatDate(
    date: string | Date,
    pattern = 'yyyy-MM-dd HH:mm'
) {
    const d = typeof date === 'string' ? parseISO(date) : date;
    return format(d, pattern);
}
