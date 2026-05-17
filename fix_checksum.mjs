import crypto from 'crypto';
import fs from 'fs';
import pg from 'pg';
import path from 'path';

async function fixAllChecksums() {
  try {
    const { Client } = pg;
    const client = new Client({
      connectionString: 'postgres://user:password@localhost:5432/db_rust_ecommerce',
    });
    
    await client.connect();

    const files = fs.readdirSync('migrations').filter(f => f.endsWith('.sql'));
    for (const file of files) {
      const version = file.split('_')[0];
      const fileBuffer = fs.readFileSync(path.join('migrations', file));
      const hash = crypto.createHash('sha384').update(fileBuffer).digest();
      
      const res = await client.query('UPDATE _sqlx_migrations SET checksum = $1 WHERE version = $2', [hash, version]);
      if (res.rowCount > 0) {
        console.log(`Updated checksum for ${file}`);
      }
    }
    
    await client.end();
  } catch (err) {
    console.error('Error fixing checksums:', err);
  }
}

fixAllChecksums();
