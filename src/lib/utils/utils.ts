export function getFileName(file_path: string | null) {
    if (file_path === null) {
        return null
    }

    return file_path.split('/').pop()
}