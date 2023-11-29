/*
 * This is a customized settings file for the SDV MIA fleet-management example.
 */

module.exports = {
    editorTheme: {
        functionExternalModules: true,
        theme: "midnight-red",
        palette: {
            categories: ['SDV', 'subflows', 'common', 'function', 'network', 'sequence', 'parser', 'storage'],
        },
        projects: {
            enabled: false,
            workflow: {
                mode: "manual"
            }
        }
    }
}
