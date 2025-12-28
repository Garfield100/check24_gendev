package com.garfield100.gendev.adapter

import android.view.LayoutInflater
import android.view.ViewGroup
import androidx.recyclerview.widget.RecyclerView
import com.garfield100.gendev.databinding.CarouselItemBinding
import com.garfield100.gendev.model.CarouselItem

class CarouselItemAdapter(private val items: List<CarouselItem>) :
    RecyclerView.Adapter<CarouselItemAdapter.ViewHolder>() {

    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): ViewHolder {
        val binding =
            CarouselItemBinding.inflate(LayoutInflater.from(parent.context), parent, false)
        return ViewHolder(binding)
    }

    override fun onBindViewHolder(holder: ViewHolder, position: Int) {
        holder.bind(items[position])
    }

    override fun getItemCount() = items.size

    inner class ViewHolder(private val binding: CarouselItemBinding) :
        RecyclerView.ViewHolder(binding.root) {

        fun bind(item: CarouselItem) {
            binding.itemTitle.text = item.title
            binding.itemDescription.text = item.description
            item.imageRes?.let {
                binding.itemImage.setImageResource(it)
            } ?: run {
                binding.itemImage.setImageDrawable(null)
            }
        }
    }
}