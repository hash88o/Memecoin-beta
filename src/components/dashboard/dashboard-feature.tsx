"use client";
import React, { useState } from 'react';
import { LineChart, XAxis, YAxis, Tooltip, Line, ResponsiveContainer } from 'recharts';
import { Wallet, BarChart3, Users, Coins, ArrowUpRight, Lock, Vote } from 'lucide-react';
import { 
  Card, 
  CardContent, 
  CardDescription, 
  CardHeader, 
  CardTitle 
} from '@/components/ui/card';

const TokenDashboard = () => {
  const [activeTab, setActiveTab] = useState('overview');
  
  // Mock data - replace with real data from your Solana program
  const priceData = [
    { date: '1/1', price: 0.00001 },
    { date: '1/2', price: 0.000012 },
    { date: '1/3', price: 0.000015 },
    { date: '1/4', price: 0.000014 },
    { date: '1/5', price: 0.000018 },
    { date: '1/6', price: 0.000022 },
    { date: '1/7', price: 0.000025 },
  ];

  const stats = {
    holders: 2854,
    totalSupply: '1,000,000,000',
    circulatingSupply: '750,000,000',
    burned: '250,000,000',
    price: '$0.000025',
    marketCap: '$25,000',
    volume24h: '$12,500'
  };

  const rewardPoolInfo = {
    totalStaked: '500,000,000',
    rewardRate: '0.5%',
    apr: '25%',
    nextReward: '12h 30m'
  };

  return (
    <div className="min-h-screen bg-gray-50 p-8">
      <div className="max-w-7xl mx-auto">
        {/* Header */}
        <div className="mb-8">
          <h1 className="text-4xl font-bold text-gray-900 mb-2">MemeToken Dashboard</h1>
          <p className="text-gray-600">Your decentralized meme token with governance and rewards</p>
        </div>

        {/* Quick Stats */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
          <Card>
            <CardContent className="pt-6">
              <div className="flex items-center justify-between">
                <div>
                  <p className="text-sm font-medium text-gray-600">Price</p>
                  <h3 className="text-2xl font-bold">{stats.price}</h3>
                </div>
                <div className="p-2 bg-green-100 rounded-full">
                  <ArrowUpRight className="w-6 h-6 text-green-600" />
                </div>
              </div>
            </CardContent>
          </Card>

          <Card>
            <CardContent className="pt-6">
              <div className="flex items-center justify-between">
                <div>
                  <p className="text-sm font-medium text-gray-600">Holders</p>
                  <h3 className="text-2xl font-bold">{stats.holders}</h3>
                </div>
                <div className="p-2 bg-blue-100 rounded-full">
                  <Users className="w-6 h-6 text-blue-600" />
                </div>
              </div>
            </CardContent>
          </Card>

          <Card>
            <CardContent className="pt-6">
              <div className="flex items-center justify-between">
                <div>
                  <p className="text-sm font-medium text-gray-600">24h Volume</p>
                  <h3 className="text-2xl font-bold">{stats.volume24h}</h3>
                </div>
                <div className="p-2 bg-purple-100 rounded-full">
                  <BarChart3 className="w-6 h-6 text-purple-600" />
                </div>
              </div>
            </CardContent>
          </Card>

          <Card>
            <CardContent className="pt-6">
              <div className="flex items-center justify-between">
                <div>
                  <p className="text-sm font-medium text-gray-600">Market Cap</p>
                  <h3 className="text-2xl font-bold">{stats.marketCap}</h3>
                </div>
                <div className="p-2 bg-orange-100 rounded-full">
                  <Coins className="w-6 h-6 text-orange-600" />
                </div>
              </div>
            </CardContent>
          </Card>
        </div>

        {/* Main Chart */}
        <Card className="mb-8">
          <CardHeader>
            <CardTitle>Price History</CardTitle>
            <CardDescription>Token price over time</CardDescription>
          </CardHeader>
          <CardContent>
            <div className="h-64">
              <ResponsiveContainer width="100%" height="100%">
                <LineChart data={priceData}>
                  <XAxis dataKey="date" />
                  <YAxis />
                  <Tooltip />
                  <Line 
                    type="monotone" 
                    dataKey="price" 
                    stroke="#8884d8" 
                    strokeWidth={2}
                  />
                </LineChart>
              </ResponsiveContainer>
            </div>
          </CardContent>
        </Card>

        {/* Token Info Grid */}
        <div className="grid grid-cols-1 md:grid-cols-2 gap-8 mb-8">
          {/* Supply Info */}
          <Card>
            <CardHeader>
              <CardTitle>Token Supply</CardTitle>
              <CardDescription>Supply and circulation details</CardDescription>
            </CardHeader>
            <CardContent>
              <div className="space-y-4">
                <div className="flex justify-between">
                  <span className="text-gray-600">Total Supply</span>
                  <span className="font-medium">{stats.totalSupply}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-600">Circulating Supply</span>
                  <span className="font-medium">{stats.circulatingSupply}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-600">Burned Tokens</span>
                  <span className="font-medium">{stats.burned}</span>
                </div>
              </div>
            </CardContent>
          </Card>

          {/* Reward Pool Info */}
          <Card>
            <CardHeader>
              <CardTitle>Reward Pool</CardTitle>
              <CardDescription>Staking and rewards information</CardDescription>
            </CardHeader>
            <CardContent>
              <div className="space-y-4">
                <div className="flex justify-between">
                  <span className="text-gray-600">Total Staked</span>
                  <span className="font-medium">{rewardPoolInfo.totalStaked}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-600">Reward Rate</span>
                  <span className="font-medium">{rewardPoolInfo.rewardRate}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-600">APR</span>
                  <span className="font-medium text-green-600">{rewardPoolInfo.apr}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-600">Next Reward</span>
                  <span className="font-medium">{rewardPoolInfo.nextReward}</span>
                </div>
              </div>
            </CardContent>
          </Card>
        </div>

        {/* Action Buttons */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          <button className="flex items-center justify-center gap-2 bg-blue-600 text-white p-4 rounded-lg hover:bg-blue-700 transition-colors">
            <Wallet className="w-5 h-5" />
            Connect Wallet
          </button>
          <button className="flex items-center justify-center gap-2 bg-purple-600 text-white p-4 rounded-lg hover:bg-purple-700 transition-colors">
            <Lock className="w-5 h-5" />
            Stake Tokens
          </button>
          <button className="flex items-center justify-center gap-2 bg-green-600 text-white p-4 rounded-lg hover:bg-green-700 transition-colors">
            <Vote className="w-5 h-5" />
            Governance
          </button>
        </div>
      </div>
    </div>
  );
};

export default TokenDashboard;