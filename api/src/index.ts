import IORedis from 'ioredis';
import express from 'express';

const connection = new IORedis({ host: 'localhost', port: 6379 });
const TASK_QUEUE = 'aura-grid-tasks';

const app = express();
app.use(express.json());

app.post('/scan', async (req, res) => {
    const { contract_id, code } = req.body;
    if (!contract_id || !code) return res.status(400).json({ error: 'Missing contract_id or code' });
    
    const task = JSON.stringify({ contract_id, code, timestamp: Date.now() });
    await connection.lpush(TASK_QUEUE, task);
    
    res.json({ status: 'queued', contract_id });
});

app.get('/health', (req, res) => {
    res.json({ status: 'active', engine: 'Aura-Grid Controller' });
});

const PORT = 3000;
app.listen(PORT, () => {
    console.log(`🚀 Aura-Grid Controller active on port ${PORT}`);
});