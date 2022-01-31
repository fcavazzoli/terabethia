import {
  PutCommand,
  GetCommand,
} from '@aws-sdk/lib-dynamodb';
import Database from './database';

class EthereumDatabase extends Database {
  async storeTransaction(txHash: string): Promise<void> {
    await this.db.send(
      new PutCommand({
        TableName: this.tableName,
        Item: {
          PrimaryKey: txHash,
          CreatedAt: Date.now(),
        },
      }),
    );
  }

  async hasTransaction(txHash: string): Promise<boolean> {
    const res = await this.db.send(
      new GetCommand({
        TableName: this.tableName,
        Key: {
          PrimaryKey: txHash,
        },
      }),
    );

    return !!res.Item;
  }

  async storeMessageHash(hash: string): Promise<void> {
    await this.db.send(
      new PutCommand({
        TableName: this.tableName,
        Item: {
          PrimaryKey: `m_${hash}`,
          CreatedAt: Date.now(),
        },
      }),
    );
  }

  async hasMessageHash(hash: string): Promise<boolean> {
    const res = await this.db.send(
      new GetCommand({
        TableName: this.tableName,
        Key: {
          PrimaryKey: `m_${hash}`,
        },
      }),
    );

    return !!res.Item;
  }
}

export default EthereumDatabase;