export type MenuItem = {
    icon: string;
    text: string;
    onClick: () => void;
} | typeof MenuItemHr;

export const MenuItemHr = {
    "text": "hr",
    "icon": undefined,
    onClick: undefined,
}
