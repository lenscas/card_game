using Godot;
using System;
using CardGame;
public class character_select : character_selectFs
{
	[Signal]
	public delegate void SelectedCharacter(int a);
}
