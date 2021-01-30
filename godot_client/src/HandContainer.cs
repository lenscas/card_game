using Godot;
using System;

using CardGame;

public class HandContainer : HandContainerFs
{
    [Signal]
    public delegate void PlayCard(int a, int b);
}

