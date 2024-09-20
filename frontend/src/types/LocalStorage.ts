export const keys = <const>[ 'cluster' ];
export type KeyType = typeof keys[number];

export const LOCAL_STORAGE_KEY_PREFIX = '21gg';
export type LocalStorageKey <T extends KeyType> = `${typeof LOCAL_STORAGE_KEY_PREFIX}-${T}`;