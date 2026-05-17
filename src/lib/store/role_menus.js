import { writable } from 'svelte/store';

// Default menu access mappings
/** @type {Object.<string, string[]>} */
const defaultRoleMenus = {
    admin: ["Dashboard", "Pembelian", "Penjualan", "Barang", "Satuan Barang", "Kategori Barang", "Pengguna", "Peran & Menu", "Pelanggan", "Supplier"],
    kasir: ["Penjualan", "Pelanggan"],
    gudang: ["Pembelian", "Barang", "Supplier"]
};

// Default custom roles list (Name, Code, Description)
const defaultRoles = [
    { name: "Administrator", code: "admin", description: "Hak akses penuh ke seluruh modul sistem" },
    { name: "Kasir (Cashier)", code: "kasir", description: "Melakukan transaksi penjualan & kasir" },
    { name: "Staf Gudang (Warehouse)", code: "gudang", description: "Mengatur data barang & restock supplier" }
];

let savedRoleMenus = defaultRoleMenus;
let savedRoles = defaultRoles;

if (typeof window !== 'undefined') {
    try {
        const storedMenus = localStorage.getItem('pos_pro_role_menus');
        if (storedMenus) {
            savedRoleMenus = JSON.parse(storedMenus);
        }
        const storedRoles = localStorage.getItem('pos_pro_roles_list');
        if (storedRoles) {
            savedRoles = JSON.parse(storedRoles);
        }
    } catch (e) {
        console.error("Error reading role menus from localStorage:", e);
    }
}

export const roleMenusStore = writable(savedRoleMenus);
export const rolesStore = writable(savedRoles);

// Sync with localStorage
roleMenusStore.subscribe(value => {
    if (typeof window !== 'undefined') {
        try {
            localStorage.setItem('pos_pro_role_menus', JSON.stringify(value));
        } catch (e) {
            console.error("Error writing role menus to localStorage:", e);
        }
    }
});

rolesStore.subscribe(value => {
    if (typeof window !== 'undefined') {
        try {
            localStorage.setItem('pos_pro_roles_list', JSON.stringify(value));
        } catch (e) {
            console.error("Error writing roles list to localStorage:", e);
        }
    }
});

/**
 * Update allowed menus for a role code
 * @param {string} roleCode 
 * @param {string[]} menus 
 */
export function updateRoleMenus(roleCode, menus) {
    roleMenusStore.update(current => {
        return {
            ...current,
            [roleCode]: menus
        };
    });
}

/**
 * Add a new role
 * @param {string} name 
 * @param {string} code 
 * @param {string} description 
 */
export function addNewRole(name, code, description) {
    const formattedCode = code.toLowerCase().trim().replace(/\s+/g, '_');
    rolesStore.update(current => {
        // Prevent duplicate codes
        if (current.some(r => r.code === formattedCode)) return current;
        return [...current, { name, code: formattedCode, description }];
    });
    // Set default permissions (empty list initially)
    roleMenusStore.update(current => {
        return { ...current, [formattedCode]: ["Dashboard"] };
    });
}

/**
 * Delete a role
 * @param {string} roleCode 
 */
export function deleteRole(roleCode) {
    if (roleCode === 'admin' || roleCode === 'kasir' || roleCode === 'gudang') return; // protect default roles
    rolesStore.update(current => current.filter(r => r.code !== roleCode));
    roleMenusStore.update(current => {
        const copy = { ...current };
        delete copy[roleCode];
        return copy;
    });
}
