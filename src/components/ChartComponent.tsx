'use client';
import React from 'react';
import { AreaChart, Area, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from 'recharts';

export default function ChartComponent({ chartData }: { chartData: any[] }) {
  return (
    <ResponsiveContainer width="100%" height="100%">
      <AreaChart data={chartData} margin={{ top: 0, right: 0, left: -20, bottom: 0 }}>
        <defs>
          <linearGradient id="color3" x1="0" y1="0" x2="0" y2="1">
            <stop offset="5%" stopColor="#06b6d4" stopOpacity={0.8}/>
            <stop offset="95%" stopColor="#06b6d4" stopOpacity={0}/>
          </linearGradient>
          <linearGradient id="color6" x1="0" y1="0" x2="0" y2="1">
            <stop offset="5%" stopColor="#a855f7" stopOpacity={0.8}/>
            <stop offset="95%" stopColor="#a855f7" stopOpacity={0}/>
          </linearGradient>
        </defs>
        <CartesianGrid strokeDasharray="3 3" stroke="#222" vertical={false} />
        <XAxis dataKey="time" hide />
        <YAxis stroke="#555" fontSize={10} tickFormatter={(v) => `${v}Hz`} />
        <Tooltip 
          contentStyle={{ backgroundColor: '#050505', border: '1px solid #bc13fe', fontFamily: 'monospace', fontSize: '10px' }}
          itemStyle={{ color: '#00f2ff' }}
        />
        <Area type="monotone" dataKey="freq3" stroke="#00f2ff" fillOpacity={1} fill="url(#color3)" />
        <Area type="monotone" dataKey="freq6" stroke="#bc13fe" fillOpacity={1} fill="url(#color6)" />
        <Area type="monotone" dataKey="freq9" stroke="#d4af37" fillOpacity={0.1} />
      </AreaChart>
    </ResponsiveContainer>
  );
}
