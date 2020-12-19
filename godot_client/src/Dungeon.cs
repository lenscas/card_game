using Godot;
using System;

using CardGame;

public class Dungeon : DungeonFs
{
	[Export]
	public string test { get; set; }
	[Signal]
	public delegate void GotDungeonLayout(string a);
}
