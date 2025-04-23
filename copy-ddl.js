const fs = require('fs');
const path = require('path');

// Source DDL directory
const srcDdlDir = path.join(__dirname, '..', 'ddl');
// Destination directory
const destDir = path.join(__dirname, 'data', 'ddl');

// Create destination directory if it doesn't exist
if (!fs.existsSync(destDir)) {
  fs.mkdirSync(destDir, { recursive: true });
  console.log(`Created directory: ${destDir}`);
}

// Copy DDL files
const ddlFiles = [
  'accounts.sql',
  'categories.sql',
  'data_logs.sql',
  'tags.sql',
  'transactions.sql',
  'transaction_tags.sql'
];

ddlFiles.forEach(file => {
  const srcPath = path.join(srcDdlDir, file);
  const destPath = path.join(destDir, file);
  
  if (fs.existsSync(srcPath)) {
    fs.copyFileSync(srcPath, destPath);
    console.log(`Copied ${file} to ${destPath}`);
  } else {
    console.error(`Source file not found: ${srcPath}`);
  }
});

console.log('DDL files copied successfully!');
