
# Blueprints Documents

The documents in this folder are included in eclipse-sdv-blueprint website. The eclipse sdv blueprints website is created with Docusaurus (https://docusaurus.io) and deployed as github pages.

- Blueprints website repository: 
  https://github.com/eclipse-sdv-blueprints/blueprints-website

- Blueprints website URL (not CNAME): 
  https://eclipse-sdv-blueprints.github.io/blueprints-website

## Including new documents in the website build
The documents in this folder are included using the **https://github.com/rdilweb/docusaurus-plugin-remote-content** plugin. Please edit and commit the **blueprints-website/docusaurus.config.js** file to include new files and images, e.g:

```javascript
plugins:[ [
    "docusaurus-plugin-remote-content",
    {
        name: "fleet-management", 
        // the base url for the markdown (gets prepended to all of the documents when fetching)
        sourceBaseUrl: "https://raw.githubusercontent.com/eclipse-sdv-blueprints/fleet-management/main/docs", 
        // the base directory to output to.
        outDir: "docs/fleet-management", 
        // the file names to download
        documents: ["introduction.md"], 
    },
], [
    "docusaurus-plugin-remote-content",
    {
        name: "fleet-management-img",
        // the base url for the markdown (gets prepended to all of the documents when fetching)
        sourceBaseUrl: "https://raw.githubusercontent.com/eclipse-sdv-blueprints/fleet-management/main/docs/img", 
        // the base directory to output to.
        outDir: "docs/fleet-management/img",
         // the file names to download
        documents: ["architecture.png"],
    },
]],
```

## Updating the website

Trigger the github action that builds the website for the eclipse sdv blueprints
