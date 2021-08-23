using Godot;
using System;

using CardGame;

public class DungeonCardContainer : DungeonCardContainerFs
{
	[Signal]
	public delegate void UpdateDungeon(int a);
}

